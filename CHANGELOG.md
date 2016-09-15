# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/) 
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.1.2] - 2016-09-15

### Added

- Parse target_vendor. `Cfg` now has a new `target_vendor` field.

## [v0.1.1] - 2016-09-15

### Changed

- `Cfg::new` now prefers shelling out to RUSTC, if set, instead of `rustc`. Rationale: Cargo passes
  this variable to build scripts with the path to the `rustc` that used to build the build script,
  which may not match the `rustc` in `PATH`.
  
## v0.1.0 - 2016-09-10

- Initial release

[Unreleased]: https://github.com/japaric/rustc-cfg/compare/v0.1.2...HEAD
[v0.1.2]: https://github.com/japaric/rustc-cfg/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/japaric/rustc-cfg/compare/v0.1.0...v0.1.1
