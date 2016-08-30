//! Target specifications at build.rs time
//!
//! # How to use
//!
//! ``` no_run
//! // build.rs
//! extern crate build_target;
//!
//! use std::env;
//!
//! use build_target::Target;
//!
//! fn main() {
//!     let target = Target::new(env::var_os("TARGET").unwrap()).unwrap();
//!
//!     if target.target_arch == "arm" {
//!          // don't compile this or that C file
//!     }
//! }
//! ```

#![deny(missing_docs)]
#![deny(warnings)]

use std::ffi::OsStr;
use std::process::Command;

macro_rules! u {
    ($e:expr) => {
        $e.unwrap_or_else(|_| panic!(stringify!($e)))
    }
}

/// Target specification
#[cfg_attr(test, derive(Debug))]
pub struct Target {
    /// Equivalent to `cfg(target_os = "..")`
    pub target_os: String,
    /// Equivalent to `cfg(unix)` or `cfg(windows)`
    pub target_family: String,
    /// Equivalent to `cfg(target_arch = "..")`
    pub target_arch: String,
    /// Equivalent to `cfg(target_endian = "..")`
    pub target_endian: String,
    /// Equivalent to `cfg(target_pointer_width = "..")`
    pub target_pointer_width: String,
    /// Equivalent to `cfg(target_env = "..")`
    pub target_env: String,
    /// Equivalent to `cfg(target_has_atomic = "..")`
    pub target_has_atomic: Vec<String>,
    /// Equivalent to `cfg(target_feature = "..")`
    pub target_feature: Vec<String>,
    _0: (),
}

impl Target {
    /// Returns the target specification of `target`
    ///
    /// # Errors
    ///
    /// Returns `Err` if `rustc` can't load the target specification for this target. This usually
    /// means that:
    ///
    /// - The target triple is wrong
    /// - This is not a "built-in" target and rustc can't find a target specification file (.json)
    ///   for this "custom" target.
    /// - `rustc` was built against an LLVM that doesn't have the backend to support this target.
    ///   This also implies that you can't generate binaries for this target anyway.
    pub fn new<S>(target: S) -> Result<Target, ()>
        where S: AsRef<OsStr>
    {
        Target::new_(target.as_ref())
    }

    fn new_(target: &OsStr) -> Result<Target, ()> {
        let output = u!(Command::new("rustc")
            .arg("--target")
            .arg(target)
            .args(&["--print", "cfg"])
            .output());

        if !output.status.success() {
            return Err(());
        }

        let spec = u!(String::from_utf8(output.stdout));
        let mut target_os = Err(());
        let mut target_family = Err(());
        let mut target_arch = Err(());
        let mut target_endian = Err(());
        let mut target_pointer_width = Err(());
        let mut target_env = Err(());
        let mut target_has_atomic = vec![];
        let mut target_feature = vec![];

        for entry in spec.lines() {
            let mut parts = entry.split('=');

            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                match key {
                    "target_os" => target_os = Ok(value.trim_matches('"').to_string()),
                    "target_family" => target_family = Ok(value.trim_matches('"').to_string()),
                    "target_arch" => target_arch = Ok(value.trim_matches('"').to_string()),
                    "target_endian" => target_endian = Ok(value.trim_matches('"').to_string()),
                    "target_pointer_width" => {
                        target_pointer_width = Ok(value.trim_matches('"').to_string())
                    }
                    "target_env" => target_env = Ok(value.trim_matches('"').to_string()),
                    "target_has_atomic" => {
                        target_has_atomic.push(value.trim_matches('"').to_string())
                    }
                    "target_feature" => target_feature.push(value.trim_matches('"').to_string()),
                    _ => {}
                }
            }
        }

        Ok(Target {
            target_os: u!(target_os),
            target_family: u!(target_family),
            target_arch: u!(target_arch),
            target_endian: u!(target_endian),
            target_pointer_width: u!(target_pointer_width),
            target_env: u!(target_env),
            target_has_atomic: target_has_atomic,
            target_feature: target_feature,
            _0: (),
        })
    }
}

#[cfg(test)]
mod test {
    use std::process::Command;

    #[test]
    fn all() {
        let targets = String::from_utf8(Command::new("rustc")
                .args(&["--print", "target-list"])
                .output()
                .unwrap()
                .stdout)
            .unwrap();

        for target in targets.lines() {
            println!("{}\n\t{:?}\n", target, ::Target::new(target));
        }
    }
}
