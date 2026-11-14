//! B-001: `mage_sim` depends on Bevy and Avian, but on nothing that opens a window, renders or
//! plays audio (ADR-0020). Checked on the dependency graph for every target platform, so a
//! platform-specific render dependency is caught too.

use std::collections::BTreeSet;
use std::process::Command;

/// Crates that only a client needs. None of them may reach the simulation.
const FORBIDDEN: &[&str] = &[
    "bevy_render",
    "bevy_core_pipeline",
    "bevy_pbr",
    "bevy_post_process",
    "bevy_anti_alias",
    "bevy_sprite_render",
    "bevy_ui_render",
    "bevy_gizmos_render",
    "bevy_solari",
    "bevy_window",
    "bevy_winit",
    "bevy_gilrs",
    "bevy_audio",
    "wgpu",
    "winit",
    "gilrs",
    "rodio",
    "cpal",
];

/// Crates the simulation is built on; without them the check above would pass vacuously.
const REQUIRED: &[&str] = &["bevy_app", "bevy_ecs", "avian3d"];

/// Names of all crates in the normal and build dependency graph of `mage_sim`.
fn dependency_names() -> BTreeSet<String> {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let output = Command::new(cargo)
        .args([
            "tree",
            "--package",
            "mage_sim",
            "--edges",
            "normal,build",
            "--target",
            "all",
            "--prefix",
            "none",
            "--format",
            "{p}",
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("cargo tree runs");
    assert!(
        output.status.success(),
        "cargo tree failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("cargo tree prints UTF-8")
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .map(str::to_owned)
        .collect()
}

#[test]
fn mage_sim_has_no_window_render_or_audio_dependency() {
    let names = dependency_names();
    let found: Vec<_> = FORBIDDEN
        .iter()
        .filter(|name| names.contains(**name))
        .collect();
    assert!(found.is_empty(), "mage_sim depends on {found:?}");
}

#[test]
fn mage_sim_builds_on_bevy_ecs_and_avian() {
    let names = dependency_names();
    let missing: Vec<_> = REQUIRED
        .iter()
        .filter(|name| !names.contains(**name))
        .collect();
    assert!(missing.is_empty(), "mage_sim lacks {missing:?}");
}
