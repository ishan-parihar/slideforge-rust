use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PlatformSpec {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: String,
    pub default_aspect_ratio: String,
    pub allowed_aspect_ratios: Vec<String>,
    pub format: String,
    pub description: String,
    pub recommended_for: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PlatformCanvas {
    pub platform: String,
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: String,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RenderCanvas {
    pub platform: String,
    /// Base rendering width (always 420 for vectoric scaling)
    pub base_width: u32,
    /// Base rendering height computed from aspect ratio
    pub base_height: u32,
    /// Target output width
    pub target_width: u32,
    /// Target output height
    pub target_height: u32,
    pub aspect_ratio: String,
    pub format: String,
    pub scale_factor: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AspectRatioSpec {
    pub ratio: String,
    pub width: u32,
    pub height: u32,
    pub format: String,
}

fn build_platforms() -> Vec<PlatformSpec> {
    vec![
        PlatformSpec {
            name: "instagram_portrait".to_string(),
            width: 1080,
            height: 1350,
            aspect_ratio: "4:5".to_string(),
            default_aspect_ratio: "4:5".to_string(),
            allowed_aspect_ratios: vec!["4:5".to_string(), "3:4".to_string(), "1:1".to_string()],
            format: "portrait".to_string(),
            description: "Instagram portrait carousel".to_string(),
            recommended_for: vec![
                "Instagram feed carousels".to_string(),
                "Product showcases".to_string(),
                "Educational content".to_string(),
            ],
        },
        PlatformSpec {
            name: "instagram_square".to_string(),
            width: 1080,
            height: 1080,
            aspect_ratio: "1:1".to_string(),
            default_aspect_ratio: "1:1".to_string(),
            allowed_aspect_ratios: vec!["1:1".to_string(), "4:5".to_string()],
            format: "square".to_string(),
            description: "Instagram square posts".to_string(),
            recommended_for: vec![
                "Instagram feed posts".to_string(),
                "Brand announcements".to_string(),
                "Quote cards".to_string(),
            ],
        },
        PlatformSpec {
            name: "instagram_story".to_string(),
            width: 1080,
            height: 1920,
            aspect_ratio: "9:16".to_string(),
            default_aspect_ratio: "9:16".to_string(),
            allowed_aspect_ratios: vec!["9:16".to_string(), "3:4".to_string()],
            format: "portrait".to_string(),
            description: "Instagram/TikTok Stories".to_string(),
            recommended_for: vec![
                "Instagram Stories".to_string(),
                "TikTok slides".to_string(),
                "Vertical short-form content".to_string(),
            ],
        },
        PlatformSpec {
            name: "tiktok_vertical".to_string(),
            width: 1080,
            height: 1920,
            aspect_ratio: "9:16".to_string(),
            default_aspect_ratio: "9:16".to_string(),
            allowed_aspect_ratios: vec!["9:16".to_string()],
            format: "portrait".to_string(),
            description: "TikTok vertical video/slides".to_string(),
            recommended_for: vec![
                "TikTok photo mode".to_string(),
                "TikTok slideshows".to_string(),
                "Reels-style content".to_string(),
            ],
        },
        PlatformSpec {
            name: "linkedin_landscape".to_string(),
            width: 1200,
            height: 627,
            aspect_ratio: "~1.9:1".to_string(),
            default_aspect_ratio: "4:5".to_string(),
            allowed_aspect_ratios: vec!["4:5".to_string(), "1:1".to_string()],
            format: "landscape".to_string(),
            description: "LinkedIn document posts".to_string(),
            recommended_for: vec![
                "LinkedIn articles".to_string(),
                "Professional carousels".to_string(),
                "B2B thought leadership".to_string(),
            ],
        },
        PlatformSpec {
            name: "twitter_card".to_string(),
            width: 1200,
            height: 675,
            aspect_ratio: "16:9".to_string(),
            default_aspect_ratio: "1:1".to_string(),
            allowed_aspect_ratios: vec!["1:1".to_string(), "4:5".to_string()],
            format: "landscape".to_string(),
            description: "Twitter/X card images".to_string(),
            recommended_for: vec![
                "Twitter/X posts".to_string(),
                "Link preview cards".to_string(),
                "Announcement images".to_string(),
            ],
        },
        PlatformSpec {
            name: "facebook_post".to_string(),
            width: 1200,
            height: 630,
            aspect_ratio: "~1.9:1".to_string(),
            default_aspect_ratio: "3:4".to_string(),
            allowed_aspect_ratios: vec!["3:4".to_string(), "4:5".to_string(), "1:1".to_string()],
            format: "landscape".to_string(),
            description: "Facebook post images".to_string(),
            recommended_for: vec![
                "Facebook feed posts".to_string(),
                "Facebook ads".to_string(),
                "Shared link previews".to_string(),
            ],
        },
        PlatformSpec {
            name: "presentation_16_9".to_string(),
            width: 1920,
            height: 1080,
            aspect_ratio: "16:9".to_string(),
            default_aspect_ratio: "16:9".to_string(),
            allowed_aspect_ratios: vec!["16:9".to_string()],
            format: "landscape".to_string(),
            description: "Presentation slides (16:9)".to_string(),
            recommended_for: vec![
                "Google Slides".to_string(),
                "PowerPoint widescreen".to_string(),
                "Keynote presentations".to_string(),
            ],
        },
        PlatformSpec {
            name: "presentation_4_3".to_string(),
            width: 1024,
            height: 768,
            aspect_ratio: "4:3".to_string(),
            default_aspect_ratio: "4:3".to_string(),
            allowed_aspect_ratios: vec!["4:3".to_string()],
            format: "landscape".to_string(),
            description: "Presentation slides (4:3)".to_string(),
            recommended_for: vec![
                "Classic PowerPoint".to_string(),
                "Projector presentations".to_string(),
                "Legacy slide formats".to_string(),
            ],
        },
    ]
}

/// Return a [`PlatformSpec`] by name, or `None` if unknown.
pub fn get_platform(name: &str) -> Option<PlatformSpec> {
    build_platforms().into_iter().find(|p| p.name == name)
}

/// Return all known platform names.
pub fn list_platforms() -> Vec<String> {
    build_platforms().into_iter().map(|p| p.name).collect()
}

/// Return all [`PlatformSpec`] definitions.
pub fn all_platforms() -> Vec<PlatformSpec> {
    build_platforms()
}

pub fn aspect_ratio_dimensions(ratio: &str) -> Option<AspectRatioSpec> {
    match ratio {
        "4:5" => Some(AspectRatioSpec {
            ratio: ratio.to_string(),
            width: 1080,
            height: 1350,
            format: "portrait".to_string(),
        }),
        "9:16" => Some(AspectRatioSpec {
            ratio: ratio.to_string(),
            width: 1080,
            height: 1920,
            format: "portrait".to_string(),
        }),
        "3:4" => Some(AspectRatioSpec {
            ratio: ratio.to_string(),
            width: 1080,
            height: 1440,
            format: "portrait".to_string(),
        }),
        "1:1" => Some(AspectRatioSpec {
            ratio: ratio.to_string(),
            width: 1080,
            height: 1080,
            format: "square".to_string(),
        }),
        "16:9" => Some(AspectRatioSpec {
            ratio: ratio.to_string(),
            width: 1920,
            height: 1080,
            format: "landscape".to_string(),
        }),
        "4:3" => Some(AspectRatioSpec {
            ratio: ratio.to_string(),
            width: 1024,
            height: 768,
            format: "landscape".to_string(),
        }),
        _ => None,
    }
}

pub fn resolve_canvas(
    platform: &str,
    aspect_ratio: Option<&str>,
) -> Result<PlatformCanvas, String> {
    let spec = get_platform(platform).ok_or_else(|| {
        let valid: Vec<String> = all_platforms().iter().map(|p| p.name.clone()).collect();
        format!(
            "Unknown platform '{}'. Valid platforms: {}",
            platform,
            valid.join(", ")
        )
    })?;
    let ratio = aspect_ratio
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(&spec.default_aspect_ratio);

    if !spec.allowed_aspect_ratios.iter().any(|r| r == ratio) {
        return Err(format!(
            "Aspect ratio '{ratio}' is not allowed for platform '{}'. Allowed: {}",
            spec.name,
            spec.allowed_aspect_ratios.join(", ")
        ));
    }

    let (width, height, format) = aspect_ratio_dimensions(ratio)
        .map(|spec| (spec.width, spec.height, spec.format))
        .unwrap_or_else(|| (spec.width, spec.height, spec.format.clone()));

    Ok(PlatformCanvas {
        platform: spec.name,
        width,
        height,
        aspect_ratio: ratio.to_string(),
        format,
    })
}

/// Resolve a render-plan: target dimensions + base (420px) dimensions + scale factor.
pub fn resolve_render_canvas(
    platform: &str,
    aspect_ratio: Option<&str>,
) -> Result<RenderCanvas, String> {
    let canvas = resolve_canvas(platform, aspect_ratio)?;
    let target_width = canvas.width;
    let target_height = canvas.height;
    let base_width: u32 = 420;
    let base_height: u32 = 525; // Always 4:5 composition
    let scale_factor = target_width as f32 / base_width as f32;

    Ok(RenderCanvas {
        platform: canvas.platform,
        base_width,
        base_height,
        target_width,
        target_height,
        aspect_ratio: canvas.aspect_ratio,
        format: canvas.format,
        scale_factor,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_defaults_and_allowed_ratios() {
        let ig = get_platform("instagram_portrait").expect("instagram portrait platform");
        assert_eq!(ig.default_aspect_ratio, "4:5");
        assert!(ig.allowed_aspect_ratios.contains(&"4:5".to_string()));
        assert!(ig.allowed_aspect_ratios.contains(&"3:4".to_string()));
        assert!(ig.allowed_aspect_ratios.contains(&"1:1".to_string()));

        let story = get_platform("instagram_story").expect("instagram story platform");
        assert_eq!(story.default_aspect_ratio, "9:16");
        assert!(story.allowed_aspect_ratios.contains(&"9:16".to_string()));

        let linkedin = get_platform("linkedin_landscape").expect("linkedin platform");
        assert_eq!(linkedin.default_aspect_ratio, "4:5");
        assert!(linkedin.allowed_aspect_ratios.contains(&"4:5".to_string()));
    }

    #[test]
    fn test_resolve_canvas_ratio_override() {
        let canvas = resolve_canvas("instagram_portrait", Some("3:4")).expect("3:4 canvas");
        assert_eq!(canvas.aspect_ratio, "3:4");
        assert_eq!((canvas.width, canvas.height), (1080, 1440));

        let square = resolve_canvas("instagram_portrait", Some("1:1")).expect("1:1 canvas");
        assert_eq!((square.width, square.height), (1080, 1080));
    }

    #[test]
    fn test_resolve_canvas_rejects_invalid_ratio_for_platform() {
        let err = resolve_canvas("instagram_story", Some("16:9")).unwrap_err();
        assert!(err.contains("not allowed"));
    }
}
