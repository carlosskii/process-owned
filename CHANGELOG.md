# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2023-05-07

### Added

- `ProcessOwnedMut` struct
  - `Deref` implementation
- `Immortal` struct
  - `Clone` implementation
  - `Deref` implementation

### Removed

- `ProcessOwned::DerefMut` implementation

## [0.1.0] - 2023-05-07

### Added

- `ProcessOwned` struct
  - `Clone` implementation
  - `Deref` implementation
  - `DerefMut` implementation