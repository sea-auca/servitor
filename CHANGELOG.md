# Changelog

All important changes for this project will be documented here.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased changes

Unreleased changes can be found in the [development](https://github.com/sea-auca/servitor/tree/dev) branch.

### [1.2.0] - 2022-04-13

### Added

- Command to retrieve user avatar
- Functional for logger to share its logfile path

### Changed

- Change `retrieve_logs` command and allow it accept zero arguments and send actual logfile as attachment.

### Deprecated

- Deprecate `retrieve_logs` command's version accepting one argument. Will be removed in the next major update.

### Fixed

- Fix channel mentions in greeting message
- Set limit for bytes written to `10000000` bytes as it should be (instead of `0` as it was incorrectly)

### [1.1.0] - 2022-04-13

### Added

- Internal log rotation at exceeding limit of bytes written in logfile (limit is set to `10000000` bytes).

### Changed

- Replace synchonous mutex for logger by async one.

### Fixed

- Fix broken metadata links in README.md
  
## [1.0.0] - 2022-04-13

### Added

- Base version of Servitor Discord bot.

[1.2.0]: https://github.com/sea-auca/servitor/compare/1.1.0..1.2.0
[1.1.0]: https://github.com/sea-auca/servitor/compare/1.0.0..1.1.0
[1.0.0]: https://github.com/sea-auca/servitor/releases/tag/1.0.0
