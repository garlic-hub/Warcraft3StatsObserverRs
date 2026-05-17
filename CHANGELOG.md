# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)

## [0.2.0]

### Added

- `ObserverHandle` type that owns the Windows file-mapping and view handles and
  releases them via `Drop`, replacing the previous leaked-handle model (#1).
- `ObserverHandle` implements `Deref` / `DerefMut` to `ObserverData`, `Send`,
  and `Sync`.
- Public constants for memory-map array bounds: `MAX_PLAYERS`, `MAX_SHOPS`,
  `MAX_HEROES`, `MAX_STRUCTURES`, `MAX_UPGRADES`, `MAX_UNITS`,
  `MAX_UNITS_IN_QUEUE`, `MAX_ITEMS`, `MAX_UPKEEP_LEVELS`.
- `PaddedString` gained `as_bytes`, `to_string_lossy`, `len`, `is_empty`, an
  `AsRef<[u8]>` impl, and `Debug` / `Clone` / `Copy` / `PartialEq` / `Eq` /
  `Hash` derives.
- `Clone`, `Copy`, `PartialEq`, `Eq`, and `Hash` derives on the public enums
  (`RacePreference`, `PlayerRace`, `PlayerType`, `PlayerGameResult`,
  `PlayerSlotState`, `AiDifficultyPreference`).
- Comprehensive rustdoc on the crate root, `ObserverData`, `ObserverHandle`,
  `PlayerInfo`, and supporting types.
- GitHub Actions CI workflow.

### Changed

- **Breaking:** Entry point moved from `ObserverData::new()` /
  `ObserverData::new_with_refresh_rate(...)` to `ObserverHandle::new()` /
  `ObserverHandle::new_with_refresh_rate(...)`. Callers now own an
  `ObserverHandle` value rather than holding a `&'static ObserverData`.
- **Breaking:** Removed the lifetime parameter from `ObserverData`
  (`ObserverData<'s>` → `ObserverData`).

### Fixed

- The Windows file-mapping and mapped view are now properly unmapped and
  closed when the handle goes out of scope; previously both handles were
  leaked for the lifetime of the process (#1).
- On `MapViewOfFile` failure, the file-mapping handle is now closed before
  returning the error.

## [0.1.1]

### Fixed

- Fixed the `stats` example.

## [0.1.0]

- Initial release.

[0.2.0]: https://github.com/garlic-hub/Warcraft3StatsObserverRs/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/garlic-hub/Warcraft3StatsObserverRs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/garlic-hub/Warcraft3StatsObserverRs/releases/tag/v0.1.0
