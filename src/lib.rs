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
    /// Runs `rustc --target <target> --print cfg` and returns the parsed output.
    ///
    /// The `target` should be a "triple" from the list of supported rustc
    /// targets. A list of supported rustc targets can be obtained using the
    /// `rustc --print target-list` command. This should not be confused with
    /// Cargo targets, i.e. binaries, `[[bin]]` and a library `[lib]` define in
    /// a package's manifest (Cargo.toml).
    pub fn of(target: &str) -> Result<Cfg, failure::Error> {
        // NOTE Cargo passes RUSTC to build scripts, prefer that over plain `rustc`.
        let output = Command::new(env::var("RUSTC").as_ref().map(|s| &**s).unwrap_or("rustc"))
            .arg("--target")
            .arg(target)
            .args(&["--print", "cfg"])
            .output()?;

        if !output.status.success() {
            return Err(failure::err_msg(String::from_utf8(output.stderr)?));
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
            target_os: target_os.ok_or_else(|| failure::err_msg("`target_os` is missing"))?,
            target_family,
            target_arch: target_arch.ok_or_else(|| failure::err_msg("`target_arch` is missing"))?,
            target_endian: target_endian
                .ok_or_else(|| failure::err_msg("`target_endian` is missing"))?,
            target_pointer_width: target_pointer_width
                .ok_or_else(|| failure::err_msg("`target_pointer_width` is missing"))?,
            target_env: target_env.ok_or_else(|| failure::err_msg("`target_env` is missing"))?,
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
