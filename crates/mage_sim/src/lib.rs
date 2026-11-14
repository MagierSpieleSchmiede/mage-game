//! The simulation of Mage Game: all gameplay as a deterministic tick chain in `FixedUpdate`.
//!
//! This crate runs headless and must build for every target platform, `wasm32-unknown-unknown`
//! included, so it depends on Bevy and Avian without windowing, rendering, audio or threads
//! (ADR-0020, ADR-0024). The test `tests/no_render_dependencies.rs` enforces this.
