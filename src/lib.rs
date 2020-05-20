//! Runs `rustc --print cfg` and parses the output
//!
//! *NOTE*: If you are in build script context you should prefer to use the [`CARGO_CFG_*`] env
//! variables that Cargo sets over this crate.
//!
//! [`CARGO_CFG_*`]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
//!
//! # Requirements
//!
//! - This crate requires `rustc` to be installed and available in the user's PATH.
//!
//! # How to use
//!
//! ```
//! extern crate rustc_cfg;
//!
//! use rustc_cfg::Cfg;
//!
//! fn main() {
//!     let cfg = Cfg::of("x86_64-unknown-linux-gnu").unwrap();
//!
//!     assert_eq!(cfg.target_arch, "x86_64");
//!     assert!(cfg.target_family.as_ref().map(|f| f == "unix").unwrap_or(false));
//! }
//! ```

#![deny(missing_docs)]
#![deny(warnings)]

use std::env;
use std::process::Command;

/// The error result of parsing the output of `rustc --print cfg`
#[derive(thiserror::Error, Debug)]
pub enum CfgError {
    /// If `rustc` exited non-successfully then the contents of stderr are returned in this error
    #[error("error when executing rustc")]
    RustcError(String),
    /// If an expected field is missing from the config
    #[error("field {0} is missing from config")]
    MissingField(&'static str),

    /// If an io related error occurred when trying to get the output of `rustc`
    #[error("error when executing rustc")]
    IoError(#[from] std::io::Error),
    /// If there was a problem parsing Utf8 from `rustc`
    #[error("error when executing rustc")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

/// The result of parsing the output of `rustc --print cfg`
#[cfg_attr(test, derive(Debug))]
pub struct Cfg {
    /// Equivalent to `cfg(target_os = "..")`
    pub target_os: String,
    /// Equivalent to `cfg(unix)` or `cfg(windows)`
    pub target_family: Option<String>,
    /// Equivalent to `cfg(target_arch = "..")`
    pub target_arch: String,
    /// Equivalent to `cfg(target_endian = "..")`
    pub target_endian: String,
    /// Equivalent to `cfg(target_pointer_width = "..")`
    pub target_pointer_width: String,
    /// Equivalent to `cfg(target_env = "..")`
    pub target_env: String,
    /// Equivalent to `cfg(target_vendor = "..")`.
    pub target_vendor: Option<String>,
    /// Equivalent to `cfg(target_has_atomic = "..")`
    pub target_has_atomic: Vec<String>,
    /// Equivalent to `cfg(target_feature = "..")`
    pub target_feature: Vec<String>,
    _extensible: (),
}

impl Cfg {
    /// Runs `rustc --print cfg <target>` and returns the parsed output
    pub fn of(target: &str) -> Result<Cfg, CfgError> {
        // NOTE Cargo passes RUSTC to build scripts, prefer that over plain `rustc`.
        let output = Command::new(env::var("RUSTC").as_ref().map(|s| &**s).unwrap_or("rustc"))
            .arg("--target")
            .arg(target)
            .args(&["--print", "cfg"])
            .output()?;

        if !output.status.success() {
            return Err(CfgError::RustcError(String::from_utf8(output.stderr)?));
        }

        let spec = String::from_utf8(output.stdout)?;
        let mut target_os = None;
        let mut target_family = None;
        let mut target_arch = None;
        let mut target_endian = None;
        let mut target_pointer_width = None;
        let mut target_env = None;
        let mut target_vendor = None;
        let mut target_has_atomic = vec![];
        let mut target_feature = vec![];

        for entry in spec.lines() {
            let mut parts = entry.split('=');

            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                match key {
                    "target_os" => target_os = Some(value.trim_matches('"').to_string()),
                    "target_family" => target_family = Some(value.trim_matches('"').to_string()),
                    "target_arch" => target_arch = Some(value.trim_matches('"').to_string()),
                    "target_endian" => target_endian = Some(value.trim_matches('"').to_string()),
                    "target_pointer_width" => {
                        target_pointer_width = Some(value.trim_matches('"').to_string())
                    }
                    "target_env" => target_env = Some(value.trim_matches('"').to_string()),
                    "target_vendor" => target_vendor = Some(value.trim_matches('"').to_string()),
                    "target_has_atomic" => {
                        target_has_atomic.push(value.trim_matches('"').to_string())
                    }
                    "target_feature" => target_feature.push(value.trim_matches('"').to_string()),
                    _ => {}
                }
            }
        }

        Ok(Cfg {
            target_os: target_os.ok_or_else(|| CfgError::MissingField("target_os"))?,
            target_family,
            target_arch: target_arch.ok_or_else(|| CfgError::MissingField("target_arch"))?,
            target_endian: target_endian
                .ok_or_else(|| CfgError::MissingField("target_endian"))?,
            target_pointer_width: target_pointer_width
                .ok_or_else(|| CfgError::MissingField("target_pointer_width"))?,
            target_env: target_env.ok_or_else(|| CfgError::MissingField("target_env"))?,
            target_vendor,
            target_has_atomic,
            target_feature,
            _extensible: (),
        })
    }
}

#[cfg(test)]
mod test {
    use std::process::Command;

    use super::Cfg;

    #[test]
    fn all() {
        let output = Command::new("rustc")
            .args(&["--print", "target-list"])
            .output()
            .unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap();

        assert!(output.status.success());

        for target in stdout.lines() {
            println!("{}\n\t{:?}\n", target, Cfg::of(target));
        }
    }
}
