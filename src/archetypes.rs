use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ArchetypePreset {
    pub alignment: String,
    pub variant: String,
    pub glass: bool,
    pub decorations: bool,
    pub padding: String,
    pub justify: String,
    pub headline_gradient: bool,
    pub accent_usage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Archetype {
    pub name: String,
    pub description: String,
    pub primary_theme: String,
    pub default_bg_style: String,
    pub slide_presets: HashMap<String, ArchetypePreset>,
}

// ---------------------------------------------------------------------------
// Default preset (fallback when archetype has no entry for a slide type)
// ---------------------------------------------------------------------------

fn default_preset() -> ArchetypePreset {
    ArchetypePreset {
        alignment: "left".to_string(),
        variant: "default".to_string(),
        glass: false,
        decorations: false,
        padding: "md".to_string(),
        justify: "start".to_string(),
        headline_gradient: false,
        accent_usage: "minimal".to_string(),
    }
}

// ---------------------------------------------------------------------------
// Builder helpers
// ---------------------------------------------------------------------------

fn preset(
    alignment: &str,
    variant: &str,
    glass: bool,
    decorations: bool,
    padding: &str,
    justify: &str,
    headline_gradient: bool,
    accent_usage: &str,
) -> ArchetypePreset {
    ArchetypePreset {
        alignment: alignment.to_string(),
        variant: variant.to_string(),
        glass,
        decorations,
        padding: padding.to_string(),
        justify: justify.to_string(),
        headline_gradient,
        accent_usage: accent_usage.to_string(),
    }
}

fn slide_map(pairs: Vec<(&str, ArchetypePreset)>) -> HashMap<String, ArchetypePreset> {
    pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect()
}

// ---------------------------------------------------------------------------
// Archetype definitions
// ---------------------------------------------------------------------------

fn build_archetypes() -> Vec<Archetype> {
    vec![
        // ── educator ────────────────────────────────────────────────────────
        Archetype {
            name: "educator".to_string(),
            description:
                "Learning-focused, clean layout with left-aligned text and minimal decorations"
                    .to_string(),
            primary_theme: "light".to_string(),
            default_bg_style: "clean_white".to_string(),
            slide_presets: slide_map(vec![
                (
                    "hero",
                    preset(
                        "center", "split", false, false, "lg", "center", false, "minimal",
                    ),
                ),
                (
                    "feature",
                    preset(
                        "left", "default", false, false, "md", "start", false, "minimal",
                    ),
                ),
                (
                    "list",
                    preset(
                        "left", "default", false, false, "md", "start", false, "minimal",
                    ),
                ),
                (
                    "quote",
                    preset(
                        "center", "quote", false, false, "lg", "center", false, "minimal",
                    ),
                ),
                (
                    "cta",
                    preset(
                        "center", "default", false, false, "md", "center", false, "moderate",
                    ),
                ),
                (
                    "stat_row",
                    preset(
                        "center", "stat", false, false, "md", "center", false, "minimal",
                    ),
                ),
                (
                    "timeline",
                    preset(
                        "left", "timeline", false, false, "md", "start", false, "minimal",
                    ),
                ),
                (
                    "comparison",
                    preset(
                        "center", "grid", false, false, "md", "center", false, "minimal",
                    ),
                ),
            ]),
        },
        // ── thought_leader ──────────────────────────────────────────────────
        Archetype {
            name: "thought_leader".to_string(),
            description: "Professional authority with bold statements and gradient headlines"
                .to_string(),
            primary_theme: "dark".to_string(),
            default_bg_style: "dark_gradient".to_string(),
            slide_presets: slide_map(vec![
                (
                    "hero",
                    preset(
                        "center", "bold", true, true, "xl", "center", true, "moderate",
                    ),
                ),
                (
                    "feature",
                    preset(
                        "left", "split", true, false, "md", "start", true, "moderate",
                    ),
                ),
                (
                    "list",
                    preset(
                        "left", "default", false, false, "md", "start", false, "moderate",
                    ),
                ),
                (
                    "quote",
                    preset("center", "quote", true, true, "xl", "center", true, "bold"),
                ),
                (
                    "cta",
                    preset("center", "bold", true, true, "lg", "center", true, "bold"),
                ),
                (
                    "stat_row",
                    preset("center", "stat", true, false, "md", "center", true, "bold"),
                ),
                (
                    "timeline",
                    preset(
                        "left", "timeline", false, false, "md", "start", false, "moderate",
                    ),
                ),
                (
                    "comparison",
                    preset(
                        "center", "grid", true, false, "md", "center", false, "moderate",
                    ),
                ),
            ]),
        },
        // ── startup_pitch ───────────────────────────────────────────────────
        Archetype {
            name: "startup_pitch".to_string(),
            description: "Vibrant, dark theme with high energy and bold numbers".to_string(),
            primary_theme: "dark".to_string(),
            default_bg_style: "vibrant_dark".to_string(),
            slide_presets: slide_map(vec![
                (
                    "hero",
                    preset("center", "bold", true, true, "xl", "center", true, "bold"),
                ),
                (
                    "feature",
                    preset("left", "split", true, true, "md", "start", true, "bold"),
                ),
                (
                    "list",
                    preset(
                        "left", "default", true, false, "md", "start", false, "moderate",
                    ),
                ),
                (
                    "quote",
                    preset("center", "quote", true, true, "lg", "center", true, "bold"),
                ),
                (
                    "cta",
                    preset("center", "bold", true, true, "xl", "center", true, "bold"),
                ),
                (
                    "stat_row",
                    preset("center", "stat", true, true, "lg", "center", true, "bold"),
                ),
                (
                    "timeline",
                    preset("left", "timeline", true, true, "md", "start", false, "bold"),
                ),
                (
                    "comparison",
                    preset("center", "grid", true, true, "md", "center", true, "bold"),
                ),
            ]),
        },
        // ── brand_storyteller ───────────────────────────────────────────────
        Archetype {
            name: "brand_storyteller".to_string(),
            description: "Emotional, imagery-forward content with warm palette".to_string(),
            primary_theme: "warm".to_string(),
            default_bg_style: "warm_gradient".to_string(),
            slide_presets: slide_map(vec![
                (
                    "hero",
                    preset(
                        "center",
                        "full_bleed",
                        false,
                        true,
                        "xl",
                        "center",
                        true,
                        "moderate",
                    ),
                ),
                (
                    "feature",
                    preset(
                        "center", "split", false, true, "lg", "center", false, "moderate",
                    ),
                ),
                (
                    "list",
                    preset(
                        "left", "default", false, false, "md", "start", false, "minimal",
                    ),
                ),
                (
                    "quote",
                    preset(
                        "center", "quote", false, true, "xl", "center", true, "moderate",
                    ),
                ),
                (
                    "cta",
                    preset("center", "bold", false, true, "lg", "center", true, "bold"),
                ),
                (
                    "stat_row",
                    preset(
                        "center", "stat", false, true, "md", "center", false, "moderate",
                    ),
                ),
                (
                    "timeline",
                    preset(
                        "left", "timeline", false, true, "md", "start", false, "moderate",
                    ),
                ),
                (
                    "comparison",
                    preset(
                        "center", "grid", false, true, "md", "center", false, "moderate",
                    ),
                ),
            ]),
        },
        // ── data_analyst ────────────────────────────────────────────────────
        Archetype {
            name: "data_analyst".to_string(),
            description: "Data-heavy, structured grids with stat cards and clean typography"
                .to_string(),
            primary_theme: "light".to_string(),
            default_bg_style: "structured_light".to_string(),
            slide_presets: slide_map(vec![
                (
                    "hero",
                    preset(
                        "left", "split", false, false, "lg", "start", false, "minimal",
                    ),
                ),
                (
                    "feature",
                    preset(
                        "left", "default", false, false, "md", "start", false, "minimal",
                    ),
                ),
                (
                    "list",
                    preset(
                        "left", "default", false, false, "md", "start", false, "minimal",
                    ),
                ),
                (
                    "quote",
                    preset(
                        "center", "quote", false, false, "md", "center", false, "minimal",
                    ),
                ),
                (
                    "cta",
                    preset(
                        "center", "default", false, false, "md", "center", false, "moderate",
                    ),
                ),
                (
                    "stat_row",
                    preset(
                        "center", "stat", false, false, "lg", "center", false, "bold",
                    ),
                ),
                (
                    "timeline",
                    preset(
                        "left", "timeline", false, false, "md", "start", false, "minimal",
                    ),
                ),
                (
                    "comparison",
                    preset(
                        "center", "grid", false, false, "lg", "center", false, "moderate",
                    ),
                ),
            ]),
        },
        // ── creator ─────────────────────────────────────────────────────────
        Archetype {
            name: "creator".to_string(),
            description: "Trendy, dynamic content with vibrant colors and bold text".to_string(),
            primary_theme: "vibrant".to_string(),
            default_bg_style: "neon_gradient".to_string(),
            slide_presets: slide_map(vec![
                (
                    "hero",
                    preset("center", "bold", true, true, "xl", "center", true, "bold"),
                ),
                (
                    "feature",
                    preset("center", "split", true, true, "lg", "center", true, "bold"),
                ),
                (
                    "list",
                    preset(
                        "left", "default", true, false, "md", "start", false, "moderate",
                    ),
                ),
                (
                    "quote",
                    preset("center", "quote", true, true, "xl", "center", true, "bold"),
                ),
                (
                    "cta",
                    preset("center", "bold", true, true, "xl", "center", true, "bold"),
                ),
                (
                    "stat_row",
                    preset("center", "stat", true, true, "lg", "center", true, "bold"),
                ),
                (
                    "timeline",
                    preset("left", "timeline", true, true, "md", "start", false, "bold"),
                ),
                (
                    "comparison",
                    preset("center", "grid", true, true, "md", "center", true, "bold"),
                ),
            ]),
        },
    ]
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Return an [`Archetype`] by name, or `None` if unknown.
pub fn get_archetype(name: &str) -> Option<Archetype> {
    build_archetypes().into_iter().find(|a| a.name == name)
}

/// Return all known archetype names.
#[allow(dead_code)]
pub fn list_archetypes() -> Vec<String> {
    build_archetypes().into_iter().map(|a| a.name).collect()
}

/// Return all [`Archetype`] definitions.
pub fn all_archetypes() -> Vec<Archetype> {
    build_archetypes()
}

/// Return the [`ArchetypePreset`] for a given slide type within an archetype.
///
/// Falls back to a sensible default preset if the archetype does not define
/// a preset for the requested `slide_type`.
pub fn get_slide_preset(archetype: &Archetype, slide_type: &str) -> ArchetypePreset {
    archetype
        .slide_presets
        .get(slide_type)
        .cloned()
        .unwrap_or_else(default_preset)
}
