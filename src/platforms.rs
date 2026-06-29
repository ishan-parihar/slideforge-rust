use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PlatformSpec {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: String,
    pub format: String,
    pub description: String,
    pub recommended_for: Vec<String>,
}

fn build_platforms() -> Vec<PlatformSpec> {
    vec![
        PlatformSpec {
            name: "instagram_portrait".to_string(),
            width: 1080,
            height: 1350,
            aspect_ratio: "4:5".to_string(),
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
