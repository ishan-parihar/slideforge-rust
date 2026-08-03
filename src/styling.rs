//! Typology styling system — anchors, operators, and the resolution algorithm.
//!
//! Model (per docs/styling-typologies-hierarchy.md):
//! - The system is an open axis space. A typology is a *named anchor* — one fully-pinned
//!   12-axis point in that space. A variant is a *formal operator transform* applied to the
//!   anchor (polarity | energy | material), never a mood-word pick. Operators compose freely.
//! - `resolve_styling` is the single entry point shared by CLI and MCP, so parity is by
//!   construction. Precedence: override > family_override > operator sequence > anchor > default.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// 2. Axis vocabulary — closed enums with concrete mappings
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Family {
    Neutral,
    Analogous,
    Complementary,
    Triadic,
    SplitComplement,
    Monochrome,
    Duotone,
}

impl Family {
    pub fn all() -> &'static [&'static str] {
        &[
            "neutral",
            "analogous",
            "complementary",
            "triadic",
            "split-complement",
            "monochrome",
            "duotone",
        ]
    }
    pub fn parse(s: &str) -> Option<Family> {
        match s {
            "neutral" => Some(Family::Neutral),
            "analogous" => Some(Family::Analogous),
            "complementary" => Some(Family::Complementary),
            "triadic" => Some(Family::Triadic),
            "split-complement" => Some(Family::SplitComplement),
            "monochrome" => Some(Family::Monochrome),
            "duotone" => Some(Family::Duotone),
            _ => None,
        }
    }
    /// The exact value derive_palette's family override switch keys on.
    pub fn scheme_key(&self) -> &'static str {
        match self {
            Family::Neutral => "neutral",
            Family::Analogous => "analogous",
            Family::Complementary => "complementary",
            Family::Triadic => "triadic",
            Family::SplitComplement => "split-complement",
            Family::Monochrome => "monochrome",
            Family::Duotone => "monochrome", // duotone collapses to mono geometry at derive_palette
        }
    }
    /// sec/tert hue offsets + chroma scales (spec section 2.3). `None` = structural family
    /// (monochrome/duotone) where the derive_palette layer applies its own math.
    pub fn offsets(&self) -> Option<(f32, f32, f32, f32)> {
        match self {
            Family::Neutral => Some((0.0, 0.44, 60.0, 0.67)),
            Family::Analogous => Some((25.0, 0.44, 50.0, 0.55)),
            Family::Complementary => Some((180.0, 0.44, 160.0, 0.44)),
            Family::Triadic => Some((120.0, 0.44, 240.0, 0.44)),
            Family::SplitComplement => Some((150.0, 0.44, -30.0, 0.50)),
            Family::Monochrome | Family::Duotone => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Surface {
    Flat,
    GlassLight,
    GlassDark,
    Frosted,
    Outline,
    GradientFill,
}

impl Surface {
    pub fn all() -> &'static [&'static str] {
        &[
            "flat",
            "glass-light",
            "glass-dark",
            "frosted",
            "outline",
            "gradient-fill",
        ]
    }
    pub fn parse(s: &str) -> Option<Surface> {
        match s {
            "flat" => Some(Surface::Flat),
            "glass-light" => Some(Surface::GlassLight),
            "glass-dark" => Some(Surface::GlassDark),
            "frosted" => Some(Surface::Frosted),
            "outline" => Some(Surface::Outline),
            "gradient-fill" => Some(Surface::GradientFill),
            _ => None,
        }
    }
    /// +1 step along the surface ladder (material operator).
    pub fn step_up(&self) -> Surface {
        match self {
            Surface::Flat => Surface::GlassLight,
            Surface::GlassLight => Surface::GlassDark,
            Surface::GlassDark => Surface::Frosted,
            Surface::Frosted => Surface::Outline,
            Surface::Outline => Surface::GradientFill,
            Surface::GradientFill => Surface::GradientFill, // ladder top
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum TypeTier {
    Compact,
    Standard,
    Airy,
}

impl TypeTier {
    pub fn all() -> &'static [&'static str] {
        &["compact", "standard", "airy"]
    }
    pub fn parse(s: &str) -> Option<TypeTier> {
        match s {
            "compact" => Some(TypeTier::Compact),
            "standard" => Some(TypeTier::Standard),
            "airy" => Some(TypeTier::Airy),
            _ => None,
        }
    }
    pub fn ratio(&self) -> f32 {
        match self {
            TypeTier::Compact => 1.15,
            TypeTier::Standard => 1.25,
            TypeTier::Airy => 1.40,
        }
    }
    /// Fixed type-scale base (px). All tiers share the 16px base; only the
    /// geometric ratio differs between compact/standard/airy.
    pub fn scale_base(&self) -> u32 {
        16
    }
    pub fn scale_ratio(&self) -> f32 {
        self.ratio()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Radius {
    Sharp,
    Rounded,
    Organic,
}

impl Radius {
    pub fn all() -> &'static [&'static str] {
        &["sharp", "rounded", "organic"]
    }
    pub fn parse(s: &str) -> Option<Radius> {
        match s {
            "sharp" => Some(Radius::Sharp),
            "rounded" => Some(Radius::Rounded),
            "organic" => Some(Radius::Organic),
            _ => None,
        }
    }
    /// (sm, md, lg) in px.
    pub fn values(&self) -> (u32, u32, u32) {
        match self {
            Radius::Sharp => (2, 4, 8),
            Radius::Rounded => (8, 12, 16),
            Radius::Organic => (12, 20, 28),
        }
    }
    pub fn step_up(&self) -> Radius {
        match self {
            Radius::Sharp => Radius::Rounded,
            Radius::Rounded => Radius::Organic,
            Radius::Organic => Radius::Organic,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Decoration {
    Calm,
    Standard,
    High,
}

impl Decoration {
    pub fn all() -> &'static [&'static str] {
        &["calm", "standard", "high"]
    }
    pub fn parse(s: &str) -> Option<Decoration> {
        match s {
            "calm" => Some(Decoration::Calm),
            "standard" => Some(Decoration::Standard),
            "high" => Some(Decoration::High),
            _ => None,
        }
    }
    /// Amplitude multiplier (float-shape density, blur strength, gradient intensity, noise opacity).
    pub fn amplitude(&self) -> f32 {
        match self {
            Decoration::Calm => 0.5,
            Decoration::Standard => 1.0,
            Decoration::High => 1.5,
        }
    }
    pub fn step_down(&self) -> Decoration {
        match self {
            Decoration::Calm => Decoration::Calm,
            Decoration::Standard => Decoration::Calm,
            Decoration::High => Decoration::Standard,
        }
    }
    pub fn step_up(&self) -> Decoration {
        match self {
            Decoration::Calm => Decoration::Standard,
            Decoration::Standard => Decoration::High,
            Decoration::High => Decoration::High,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Weight {
    Low,
    Normal,
    High,
}

impl Weight {
    pub fn all() -> &'static [&'static str] {
        &["low", "normal", "high"]
    }
    pub fn parse(s: &str) -> Option<Weight> {
        match s {
            "low" => Some(Weight::Low),
            "normal" => Some(Weight::Normal),
            "high" => Some(Weight::High),
            _ => None,
        }
    }
    /// (heading, body) weights.
    pub fn values(&self) -> (u16, u16) {
        match self {
            Weight::Low => (400, 500),
            Weight::Normal => (500, 700),
            Weight::High => (700, 900),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Case {
    Sentence,
    UpperHeadlines,
    AllCaps,
}

impl Case {
    pub fn all() -> &'static [&'static str] {
        &["sentence", "upper-headlines", "all-caps"]
    }
    pub fn parse(s: &str) -> Option<Case> {
        match s {
            "sentence" => Some(Case::Sentence),
            "upper-headlines" => Some(Case::UpperHeadlines),
            "all-caps" => Some(Case::AllCaps),
            _ => None,
        }
    }
    /// CSS text-transform for headings.
    pub fn css(&self) -> &'static str {
        match self {
            Case::Sentence => "none",
            Case::UpperHeadlines => "uppercase",
            Case::AllCaps => "uppercase",
        }
    }
    pub fn letter_spacing(&self) -> &'static str {
        match self {
            Case::Sentence => "normal",
            Case::UpperHeadlines => "0.02em",
            Case::AllCaps => "0.06em",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Tracking {
    Tight,
    Normal,
    Wide,
}

impl Tracking {
    pub fn all() -> &'static [&'static str] {
        &["tight", "normal", "wide"]
    }
    pub fn parse(s: &str) -> Option<Tracking> {
        match s {
            "tight" => Some(Tracking::Tight),
            "normal" => Some(Tracking::Normal),
            "wide" => Some(Tracking::Wide),
            _ => None,
        }
    }
    /// CSS letter-spacing for headings (em).
    pub fn css(&self) -> &'static str {
        match self {
            Tracking::Tight => "-0.005em",
            Tracking::Normal => "0em",
            Tracking::Wide => "0.02em",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum VariantOp {
    Polarity,
    Energy,
    Material,
}

impl VariantOp {
    pub fn all() -> &'static [&'static str] {
        &["polarity", "energy", "material"]
    }
    pub fn parse(s: &str) -> Option<VariantOp> {
        match s {
            "polarity" => Some(VariantOp::Polarity),
            "energy" => Some(VariantOp::Energy),
            "material" => Some(VariantOp::Material),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// 3. AxisSet — all 12 axes, fully pinned
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AxisSet {
    /// Font pairing id (11 values, spec 2.1).
    pub font: String,
    /// Primary hue bias in degrees (spec 2.2).
    pub hue: f32,
    /// Color-scheme family (spec 2.3).
    pub family: Family,
    /// Preset id (9 values, spec 2.4).
    pub preset: String,
    /// Background style (5 values, spec 2.5).
    pub bg: String,
    /// Card surface treatment (spec 2.6).
    pub surface: Surface,
    /// Type-scale tier (spec 2.7).
    pub tier: TypeTier,
    /// Corner radius (spec 2.8).
    pub radius: Radius,
    /// Decoration amplitude (spec 2.9).
    pub decor: Decoration,
    /// Font weight register (spec 2.10).
    pub weight: Weight,
    /// Heading text-transform (spec 2.11).
    pub case: Case,
    /// Heading letter-spacing (spec 2.12).
    pub tracking: Tracking,
}

impl AxisSet {
    pub fn set(&mut self, axis: &str, val: &str) -> Result<(), String> {
        match axis {
            "font" | "font-pairing" => self.font = val.to_string(),
            "hue" => {
                self.hue = val
                    .parse::<f32>()
                    .map_err(|_| format!("hue must be a float, got {val}"))?
            }
            "family" | "color-scheme" => {
                self.family = Family::parse(val)
                    .ok_or_else(|| format!("unknown family {val}"))?
            }
            "preset" => self.preset = val.to_string(),
            "bg" | "bg-style" => self.bg = val.to_string(),
            "surface" => {
                self.surface = Surface::parse(val)
                    .ok_or_else(|| format!("unknown surface {val}"))?
            }
            "type-tier" | "tier" => {
                self.tier = TypeTier::parse(val)
                    .ok_or_else(|| format!("unknown type-tier {val}"))?
            }
            "radius" => {
                self.radius = Radius::parse(val)
                    .ok_or_else(|| format!("unknown radius {val}"))?
            }
            "decoration" | "decor" => {
                self.decor = Decoration::parse(val)
                    .ok_or_else(|| format!("unknown decoration {val}"))?
            }
            "weight" => {
                self.weight = Weight::parse(val)
                    .ok_or_else(|| format!("unknown weight {val}"))?
            }
            "case" => {
                self.case = Case::parse(val)
                    .ok_or_else(|| format!("unknown case {val}"))?
            }
            "tracking" => {
                self.tracking = Tracking::parse(val)
                    .ok_or_else(|| format!("unknown tracking {val}"))?
            }
            _ => return Err(format!("unknown styling axis {axis}")),
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// 4. Typology anchors — full 12-axis pin (spec section 3)
// ---------------------------------------------------------------------------

fn base_axes(id: &str) -> AxisSet {
    let base = AxisSet {
        font: "modern".to_string(),
        hue: 0.0,
        family: Family::Neutral,
        preset: "tonal_spot".to_string(),
        bg: "light".to_string(),
        surface: Surface::GlassLight,
        tier: TypeTier::Standard,
        radius: Radius::Rounded,
        decor: Decoration::Standard,
        weight: Weight::Normal,
        case: Case::Sentence,
        tracking: Tracking::Normal,
    };
    match id {
        "editorial" => AxisSet {
            font: "editorial".into(),
            hue: 0.0,
            family: Family::Neutral,
            preset: "tonal_spot".into(),
            bg: "light".into(),
            surface: Surface::GlassLight,
            tier: TypeTier::Airy,
            radius: Radius::Sharp,
            decor: Decoration::Calm,
            weight: Weight::Normal,
            case: Case::Sentence,
            tracking: Tracking::Normal,
        },
        "startup" => AxisSet {
            font: "modern".into(),
            hue: 0.0,
            family: Family::Neutral,
            preset: "tonal_spot".into(),
            bg: "gradient".into(),
            surface: Surface::GlassLight,
            tier: TypeTier::Standard,
            radius: Radius::Rounded,
            decor: Decoration::Standard,
            weight: Weight::Normal,
            case: Case::UpperHeadlines,
            tracking: Tracking::Tight,
        },
        "technical" => AxisSet {
            font: "technical".into(),
            hue: -10.0,
            family: Family::SplitComplement,
            preset: "content".into(),
            bg: "dark".into(),
            surface: Surface::Flat,
            tier: TypeTier::Compact,
            radius: Radius::Sharp,
            decor: Decoration::Calm,
            weight: Weight::Normal,
            case: Case::UpperHeadlines,
            tracking: Tracking::Tight,
        },
        "brutalist" => AxisSet {
            font: "bold".into(),
            hue: 30.0,
            family: Family::Complementary,
            preset: "vibrant".into(),
            bg: "dark".into(),
            surface: Surface::Outline,
            tier: TypeTier::Compact,
            radius: Radius::Sharp,
            decor: Decoration::High,
            weight: Weight::High,
            case: Case::AllCaps,
            tracking: Tracking::Wide,
        },
        "luxury" => AxisSet {
            font: "luxury".into(),
            hue: 15.0,
            family: Family::Analogous,
            preset: "neutral".into(),
            bg: "light".into(),
            surface: Surface::Flat,
            tier: TypeTier::Airy,
            radius: Radius::Sharp,
            decor: Decoration::Calm,
            weight: Weight::Low,
            case: Case::UpperHeadlines,
            tracking: Tracking::Wide,
        },
        "playful" => AxisSet {
            font: "rounded".into(),
            hue: 60.0,
            family: Family::Triadic,
            preset: "fruit_salad".into(),
            bg: "mesh".into(),
            surface: Surface::GlassLight,
            tier: TypeTier::Standard,
            radius: Radius::Organic,
            decor: Decoration::High,
            weight: Weight::Normal,
            case: Case::Sentence,
            tracking: Tracking::Normal,
        },
        "vintage" => AxisSet {
            font: "vintage".into(),
            hue: -20.0,
            family: Family::Monochrome,
            preset: "neutral".into(),
            bg: "light".into(),
            surface: Surface::Flat,
            tier: TypeTier::Standard,
            radius: Radius::Organic,
            decor: Decoration::Calm,
            weight: Weight::Normal,
            case: Case::Sentence,
            tracking: Tracking::Normal,
        },
        "data" => AxisSet {
            font: "data".into(),
            hue: 10.0,
            family: Family::SplitComplement,
            preset: "content".into(),
            bg: "dark".into(),
            surface: Surface::Flat,
            tier: TypeTier::Compact,
            radius: Radius::Sharp,
            decor: Decoration::Calm,
            weight: Weight::Normal,
            case: Case::UpperHeadlines,
            tracking: Tracking::Tight,
        },
        "nature" => AxisSet {
            font: "warm".into(),
            hue: 150.0,
            family: Family::Analogous,
            preset: "content".into(),
            bg: "mesh".into(),
            surface: Surface::GlassLight,
            tier: TypeTier::Airy,
            radius: Radius::Organic,
            decor: Decoration::Calm,
            weight: Weight::Normal,
            case: Case::Sentence,
            tracking: Tracking::Normal,
        },
        "nightlife" => AxisSet {
            font: "nightlife".into(),
            hue: 60.0,
            family: Family::Complementary,
            preset: "vibrant".into(),
            bg: "hero".into(),
            surface: Surface::GlassDark,
            tier: TypeTier::Standard,
            radius: Radius::Rounded,
            decor: Decoration::High,
            weight: Weight::Normal,
            case: Case::UpperHeadlines,
            tracking: Tracking::Tight,
        },
        _ => base,
    }
}

pub fn typology_ids() -> &'static [&'static str] {
    &[
        "editorial",
        "startup",
        "technical",
        "brutalist",
        "luxury",
        "playful",
        "vintage",
        "data",
        "nature",
        "nightlife",
    ]
}

// ---------------------------------------------------------------------------
// 5. Variant operators (spec section 4)
// ---------------------------------------------------------------------------

fn apply_op(s: &mut AxisSet, op: VariantOp) {
    match op {
        VariantOp::Polarity => {
            // bg light↔dark; gradient/mesh/hero → dark; dark → light
            s.bg = match s.bg.as_str() {
                "dark" => "light".to_string(),
                "light" => "dark".to_string(),
                _ => "dark".to_string(),
            };
            // surface glass-light↔glass-dark; flat/outline unchanged
            s.surface = match s.surface {
                Surface::GlassLight => Surface::GlassDark,
                Surface::GlassDark => Surface::GlassLight,
                other => other,
            };
            s.preset = "neutral".to_string();
            s.decor = s.decor.step_down();
        }
        VariantOp::Energy => {
            s.family = match s.family {
                Family::Neutral => Family::Analogous,
                Family::Analogous => Family::Complementary,
                Family::Complementary => Family::Triadic,
                Family::SplitComplement => Family::Complementary,
                other => other, // monochrome/duotone unchanged
            };
            s.decor = s.decor.step_up();
            s.preset = "vibrant".to_string();
        }
        VariantOp::Material => {
            s.surface = s.surface.step_up();
            s.radius = s.radius.step_up();
        }
    }
}

// ---------------------------------------------------------------------------
// 6. Resolution algorithm — single entry point
// ---------------------------------------------------------------------------

/// Resolve a full 12-axis styling set.
///
/// Precedence: `override > family_override > operator sequence > anchor > system default`.
pub fn resolve_styling(
    anchor: &str,
    ops: &[VariantOp],
    family_override: Option<Family>,
    overrides: &[(&str, &str)],
) -> Result<AxisSet, String> {
    let mut s = base_axes(anchor);
    for op in ops {
        apply_op(&mut s, *op);
    }
    if let Some(f) = family_override {
        s.family = f;
    }
    for (axis, val) in overrides {
        s.set(axis, val)?;
    }
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_anchor_is_a_complete_tuple() {
        for id in typology_ids() {
            let s = base_axes(id);
            // 12 axes must be specified; font/preset/bg are non-empty strings
            assert!(!s.font.is_empty(), "{id}: font empty");
            assert!(!s.preset.is_empty(), "{id}: preset empty");
            assert!(!s.bg.is_empty(), "{id}: bg empty");
        }
    }

    #[test]
    fn typology_ids_are_unique() {
        let ids = typology_ids();
        let mut sorted = ids.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate typology ids");
    }

    #[test]
    fn operators_compose_and_are_total() {
        let base = base_axes("editorial");
        for op in [VariantOp::Polarity, VariantOp::Energy, VariantOp::Material] {
            let mut s = base.clone();
            apply_op(&mut s, op);
            // ops never produce an empty axis
            assert!(!s.bg.is_empty());
            assert!(!s.font.is_empty());
        }
        // composition: material ∘ polarity
        let mut s = base.clone();
        apply_op(&mut s, VariantOp::Polarity);
        apply_op(&mut s, VariantOp::Material);
        assert!(!s.bg.is_empty());
    }

    #[test]
    fn resolve_styling_precedence_override_wins() {
        let s = resolve_styling(
            "editorial",
            &[VariantOp::Polarity],
            Some(Family::Complementary),
            &[("surface", "outline")],
        )
        .unwrap();
        assert_eq!(s.surface, Surface::Outline);
        assert_eq!(s.family, Family::Complementary);
        // polarity flipped bg to dark
        assert_eq!(s.bg, "dark");
    }

    #[test]
    fn family_offsets_match_spec() {
        assert_eq!(Family::Neutral.offsets(), Some((0.0, 0.44, 60.0, 0.67)));
        assert_eq!(Family::Complementary.offsets(), Some((180.0, 0.44, 160.0, 0.44)));
        assert_eq!(Family::Monochrome.offsets(), None);
        assert_eq!(Family::Duotone.offsets(), None);
    }

    #[test]
    fn unknown_override_rejected() {
        let r = resolve_styling("editorial", &[], None, &[("nope", "x")]);
        assert!(r.is_err());
    }
}
