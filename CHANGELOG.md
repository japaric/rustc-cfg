# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.5.0] - 2022-07-06

### Changed

- [breaking-change] error handling has been changed to use `thiserror` (`std::error::Error`) instead
  of `failure::Error`

## [v0.4.0] - 2018-10-27

### Changed

- [breaking-change] `Cfg::new` has been renamed to `Cfg::of`, the error type
  of its return type has changed to `failure::Error` and the type of its first
  argument is now `&str`.

- [breaking-change] `rustc` version support has been increased to a recent
  release.

## [v0.3.0] - 2016-12-30

### Changed

- [breaking-change] `target_family` is now optional (`Option<String>`).

## [v0.2.0] - 2016-10-02

### Changed

- `Cfg::new` now returns a `Result<Cfg, String>`. This should result (no pun intended) in more
  helpful messages when one `unwrap`s its return value.

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

[Unreleased]: https://github.com/japaric/rustc-cfg/compare/v0.5.0...HEAD
[v0.5.0]: https://github.com/japaric/rustc-cfg/compare/v0.4.0...v0.5.0
[v0.4.0]: https://github.com/japaric/rustc-cfg/compare/v0.3.0...v0.4.0
[v0.3.0]: https://github.com/japaric/rustc-cfg/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/japaric/rustc-cfg/compare/v0.1.2...v0.2.0
[v0.1.2]: https://github.com/japaric/rustc-cfg/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/japaric/rustc-cfg/compare/v0.1.0...v0.1.1
