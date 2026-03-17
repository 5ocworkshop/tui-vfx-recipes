# Changelog

All notable changes to this project will be documented in this file.

This project follows [Semantic Versioning](https://semver.org/).

## 0.2.2 — 2026-03-17

### Fixed
- **Dependency floor too low for `charset_noise` support:** v0.2.1 declared `tui-vfx-* = "0.2"` which allowed Cargo to resolve versions prior to 0.2.5, where the `charset_noise` content effect variant does not exist. Loading the `torch_flame` recipe would fail with a parse error (`unknown variant charset_noise`) if the solver chose `tui-vfx-content` < 0.2.5. All `tui-vfx-*` dependency minimums are now pinned to 0.2.5.

## 0.2.1 — 2026-03-17

### Added
- **CharsetNoise content effect support:** Requires `tui-vfx-content` 0.2.5+. The `charset_noise` content effect type is now available in recipe JSON via the upstream `ContentEffect` enum. No code changes in this crate — the variant deserializes automatically through the existing `ContentEffect` import.

### Added (recipes)
- **recipes/torch_flame.json (v4.0.0):** Slow-burning torch fire recipe using `charset_noise` for living braille density variation with vertical gradient (flickering sparse tips, solid dense base), NeonFlicker shader for per-cell brightness flicker, and sine_wave sampler for heat shimmer.

### Fixed
- **Sampler time routing during dwell:** Samplers (pendulum, sine_wave, ripple, etc.) now receive `loop_t` instead of frozen `phase_progress` during the dwell phase. Previously, dwell samplers were effectively frozen because `phase_progress` advances at elapsed/auto_dismiss_duration (e.g., 0.0067 per 4 seconds with a 600s auto-dismiss). Filters already received `loop_t`; samplers were missed. This fix aligns sampler time routing with filter time routing.

### Changed
- Bump `tui-vfx-*` dependency minimum to 0.2.5 (for `CharsetNoise` support).

## 0.2.0 — 2026-03-13

Initial crates.io release.
