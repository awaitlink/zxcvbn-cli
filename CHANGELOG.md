# Changelog
All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.3] - 2023-01-21
### Changed
- Upgraded dependencies (including addressing [`GHSA-f85w-wvc7-crwc`](https://github.com/advisories/GHSA-f85w-wvc7-crwc)).

## [2.0.2] - 2022-06-06
### Changed
- Upgraded dependencies.

## [2.0.1] - 2020-10-05
### Changed
- Updated `zxcvbn-rs` and other dependencies.

## [2.0.0] - 2019-11-11
### Changed
- Updated `zxcvbn-rs` version.
- `dictionary` name formatting changed due to change in `zxcvbn-rs`.

## [1.1.0] - 2019-06-03
### Added
- Flag `-s` (`--secure`) for hiding password from output (hides `sequence` as well).

## [1.0.1] - 2019-06-01
### Changed
- Remove ASCII-ness check: non-ASCII input works too.

## [1.0.0] - 2019-06-01
### First Release

[2.0.3]: https://github.com/u32i64/zxcvbn-cli/compare/v2.0.2...v2.0.3
[2.0.2]: https://github.com/u32i64/zxcvbn-cli/compare/v2.0.1...v2.0.2
[2.0.1]: https://github.com/u32i64/zxcvbn-cli/compare/v2.0.0...v2.0.1
[2.0.0]: https://github.com/u32i64/zxcvbn-cli/compare/v1.1.0...v2.0.0
[1.1.0]: https://github.com/u32i64/zxcvbn-cli/compare/v1.0.1...v1.1.0
[1.0.1]: https://github.com/u32i64/zxcvbn-cli/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/u32i64/zxcvbn-cli/releases/tag/v1.0.0
