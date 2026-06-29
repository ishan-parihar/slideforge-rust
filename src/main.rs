use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use serde_json::json;
use rand::seq::SliceRandom;
use rand::thread_rng;

mod design_system;
mod slides;
mod export;
mod mcp_server;
mod blocks;
mod effects;
mod layouts;
mod components;
mod slide_registry;
mod validate;
mod platforms;
mod archetypes;

#[derive(Parser)]
#[command(name = "slideforge", version = "0.1.0")]
#[command(about = "SlideForge CLI & MCP Server", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the Model Context Protocol (MCP) server
    Mcp,
    /// Generate design system tokens from a primary color
    ConfigureDesign {
        /// Primary brand color in hex (e.g. #4F46E5)
        primary: String,
        /// Brand visual style (editorial, modern, warm, bold, rounded, classic, technical)
        #[arg(long, default_value = "modern")]
        style: String,
        /// Color palette preset (tonal_spot, vibrant, neutral, monochrome, expressive)
        #[arg(long, default_value = "tonal_spot")]
        preset: String,
        /// Output path for JSON tokens file
        #[arg(long, default_value = "design_tokens.json")]
        output: String,
    },
    /// Export rendered HTML slides to a directory of PNGs
    Export {
        /// Path to rendered HTML carousel file
        html: String,
        /// Output directory for PNG slides
        #[arg(long, default_value = "./exports")]
        output_dir: String,
        /// Total slides count
        #[arg(long)]
        slides: usize,
        /// Platform preset: instagram_portrait, instagram_square, instagram_story,
        /// tiktok_vertical, linkedin_landscape, twitter_card, facebook_post,
        /// presentation_16_9, presentation_4_3
        #[arg(long, default_value = "instagram_portrait")]
        preset: String,
    },
    /// List all available slide types with descriptions
    ListSlides,
    /// List all available export platform presets
    ListPlatforms,
    /// List all available brand archetypes
    ListArchetypes,
    /// Run an exhaustive full-scope test generating 23 carousels covering all archetypes, themes, and slide types
    TestFullScope {
        /// Output directory for test HTML files
        #[arg(long, default_value = "./test-drafts/full-scope-test-output-rust")]
        output_dir: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::ConfigureDesign { primary, style, preset, output }) => {
            println!("Generating design system for {}...", primary);
            let tokens = design_system::derive_palette(
                primary,
                style,
                16,
                1.25,
                preset,
                "",
                None,
                None,
                None,
            )?;
            let json = serde_json::to_string_pretty(&tokens)?;
            fs::write(output, json)?;
            println!("Design tokens saved to {}", output);
        }
        Some(Commands::Export { html, output_dir, slides, preset }) => {
            let (width, height) = platforms::get_platform(preset)
                .map(|p| (p.width, p.height))
                .unwrap_or_else(|| {
                    match preset.as_str() {
                        "linkedin" => (1200, 1200),
                        "tiktok" | "story" => (1080, 1920),
                        "facebook" => (1200, 630),
                        "square" => (1080, 1080),
                        _ => (1080, 1350),
                    }
                });
            println!(
                "Exporting {} slides from {} → {} at {}×{} (preset: {})...",
                slides, html, output_dir, width, height, preset
            );
            let paths = export::export_slides(html, output_dir, *slides, width, height).await?;
            println!("Export complete! Slides saved:");
            for p in paths {
                println!(" - {}", p);
            }
        }
        Some(Commands::ListSlides) => {
            let types = slide_registry::list_slide_types();
            println!("Available slide types ({}):", types.len());
            for t in &types {
                if let Some(info) = slide_registry::get_slide_type_info(t) {
                    let desc = info["description"].as_str().unwrap_or("");
                    println!("  {:<25} {}", t, desc);
                }
            }
        }
        Some(Commands::ListPlatforms) => {
            let all = platforms::all_platforms();
            println!("Available export platforms ({}):", all.len());
            for p in &all {
                println!("  {:<25} {}×{} ({})", p.name, p.width, p.height, p.aspect_ratio);
            }
        }
        Some(Commands::ListArchetypes) => {
            let all = archetypes::all_archetypes();
            println!("Available archetypes ({}):", all.len());
            for a in &all {
                println!("  {:<20} {}", a.name, a.description);
            }
        }
        Some(Commands::TestFullScope { output_dir }) => {
            run_full_scope_test(output_dir)?;
        }
        Some(Commands::Mcp) | None => {
            println!("Starting SlideForge MCP server (stdio)...");
            mcp_server::run_server().await?;
        }
    }

    Ok(())
}

fn run_full_scope_test(output_dir_str: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = Path::new(output_dir_str);
    fs::create_dir_all(output_dir)?;

    println!("Starting Rust native full-scope testing...");

    let slide_types = vec![
        "hero", "feature", "list", "quote", "cta", "comparison", "stat_row",
        "timeline", "callout", "split_features", "grid_cards",
        "headline_subheadline", "definition", "text_block"
    ];

    let archetypes_list = vec![
        "educator", "thought_leader", "startup_pitch", "brand_storyteller",
        "data_analyst", "creator"
    ];

    let platforms_list = vec![
        "instagram_portrait", "instagram_square", "instagram_story",
        "tiktok_vertical", "linkedin_landscape"
    ];

    let brand_colors = vec![
        "#6366F1", "#EF4444", "#10B981", "#F59E0B", "#8B5CF6", "#EC4899", "#06B6D4", "#84CC16"
    ];

    let themes = vec![
        "editorial", "bold", "minimal", "dark", "vibrant", "natural"
    ];

    let mut rng = thread_rng();
    let mut success = 0;
    let mut failed = 0;

    for idx in 0..23 {
        let archetype = archetypes_list[idx % archetypes_list.len()];
        let platform = platforms_list[idx % platforms_list.len()];
        let color = brand_colors[idx % brand_colors.len()];
        let theme = themes[idx % themes.len()];
        let carousel_id = idx + 1;

        // Choose 4 random slide types
        let mut chosen_types = slide_types.clone();
        chosen_types.shuffle(&mut rng);
        let carousel_types = &chosen_types[0..4];

        println!(
            "[{}/23] Testing archetype: {}, platform: {}, color: {}, theme: {} with slides: {:?}",
            carousel_id, archetype, platform, color, theme, carousel_types
        );

        // 1. Derive tokens
        let tokens = design_system::derive_palette(
            color,
            "modern",
            16,
            1.25,
            "tonal_spot",
            theme,
            None,
            None,
            None,
        );

        let tokens = match tokens {
            Ok(t) => t,
            Err(e) => {
                println!("  -> Failed to derive palette: {}", e);
                failed += 1;
                continue;
            }
        };

        // 2. Generate slides
        let mut slides_specs = Vec::new();
        for st in carousel_types {
            let rand_bg = *["light", "dark", "gradient"].choose(&mut rng).unwrap();
            let params = match *st {
                "hero" => json!({
                    "headline": "Introducing AI Analytics",
                    "subheadline": "Transform your data into insights",
                    "badge": "NEW",
                    "bg_style": rand_bg
                }),
                "cta" => json!({
                    "headline": "Ready to Get Started?",
                    "button_text": "Start Free Trial",
                    "button_url": "#",
                    "subtext": "No credit card required",
                    "bg_style": rand_bg
                }),
                "feature" => json!({
                    "icon": "🚀",
                    "title": "Lightning Fast",
                    "description": "10x faster processing",
                    "number": "10x",
                    "bg_style": rand_bg
                }),
                "list" => json!({
                    "title": "Key Benefits",
                    "items": [
                        {"title": "Automated workflows", "description": "Save 5+ hours weekly"},
                        {"title": "Real-time insights", "description": "Never miss a critical event"}
                    ],
                    "bg_style": rand_bg
                }),
                "quote" => json!({
                    "quote": "This tool transformed how we work. The results are incredible.",
                    "author": "Sarah Chen",
                    "role": "VP Engineering",
                    "bg_style": rand_bg
                }),
                "comparison" => json!({
                    "title": "Why Choose Us?",
                    "left_label": "Traditional",
                    "right_label": "AI-Powered",
                    "left_items": ["Manual processing", "Hours of work"],
                    "right_items": ["Automated", "Instant results"],
                    "bg_style": rand_bg
                }),
                "stat_row" => json!({
                    "title": "Platform Metrics",
                    "stats": [
                        {"value": "10M+", "label": "Data Points"},
                        {"value": "99.9%", "label": "Uptime"}
                    ],
                    "bg_style": rand_bg
                }),
                "timeline" => json!({
                    "title": "Implementation Roadmap",
                    "steps": [
                        {"title": "Discovery", "description": "Analyze needs"},
                        {"title": "Setup", "description": "Configure platform"}
                    ],
                    "bg_style": rand_bg
                }),
                "callout" => json!({
                    "title": "Pro Tip",
                    "text": "Start with a pilot program to validate.",
                    "icon": "💡",
                    "variant": "info",
                    "bg_style": rand_bg
                }),
                "split_features" => json!({
                    "title": "Performance",
                    "features": [
                        {"icon": "⚡", "title": "Sub-100ms latency", "description": "Global edge servers"}
                    ],
                    "bg_style": rand_bg
                }),
                "grid_cards" => json!({
                    "title": "Platform Capabilities",
                    "cards": [
                        {"icon": "🤖", "title": "AI Models", "description": "Custom LLMs"},
                        {"icon": "📊", "title": "Analytics", "description": "Visual reports"}
                    ],
                    "bg_style": rand_bg
                }),
                "headline_subheadline" => json!({
                    "headline": "The Next Chapter",
                    "subheadline": "Announcing our Series B round.",
                    "bg_style": rand_bg
                }),
                "definition" => json!({
                    "term": "Responsive Web Design",
                    "definition": "An approach to web design that makes web pages render well.",
                    "context": "Front-end Development",
                    "bg_style": rand_bg
                }),
                "text_block" => json!({
                    "title": "A Guide to Modularity",
                    "body": "Modularity is the art of breaking a system into self-contained modules.",
                    "bg_style": rand_bg
                }),
                _ => json!({})
            };

            // Inject background_image 25% of the time (matching Python test)
            let mut params = params;
            if rand::random::<f32>() < 0.25 {
                let test_images = [
                    "https://images.unsplash.com/photo-1451187580459-43490279c0fa",
                    "https://images.unsplash.com/photo-1518770660439-4636190af475",
                    "https://images.unsplash.com/photo-1506748686214-e9df14d4d9d0",
                ];
                let img_url = *test_images.choose(&mut rng).unwrap();
                let img_opacity: f32 = 0.15 + rand::random::<f32>() * 0.30;
                if let Some(obj) = params.as_object_mut() {
                    obj.insert("background_image".to_string(), json!(img_url));
                    obj.insert("image_opacity".to_string(), json!(img_opacity));
                }
            }

            let bg_style = params["bg_style"].as_str().unwrap_or("light");

            let slide_val = components::dispatch_slide(st, &tokens, &params, bg_style, theme, archetype);
            match slide_val {
                Ok(val) => {
                    let spec = serde_json::from_value::<slides::SlideSpec>(val);
                    match spec {
                        Ok(s) => slides_specs.push(s),
                        Err(e) => println!("  -> Failed to parse slide spec: {}", e),
                    }
                }
                Err(e) => println!("  -> Failed to generate slide {}: {}", st, e),
            }
        }

        // 3. Render carousel
        let spec = slides::CarouselSpec {
            slides: slides_specs,
            css_variables: tokens.to_css_variables(),
            google_fonts_url: tokens.google_fonts_url.clone(),
            heading_font: tokens.heading_font.clone(),
            body_font: tokens.body_font.clone(),
            brand_name: "NexusAI".to_string(),
            brand_handle: "@nexusai".to_string(),
            topic: format!("Rust Scope Test: {}", carousel_id),
            url: "https://nexusai.io".to_string(),
            hashtags: vec!["#nexusai".to_string(), format!("#{}", archetype), format!("#{}", theme)],
            show_progress: true,
            visual_theme: theme.to_string(),
        };

        let html = slides::render_carousel_html(&spec);
        let file_name = format!("carousel_{}_{}.html", carousel_id, archetype);
        let file_path = output_dir.join(file_name);
        
        match fs::write(&file_path, html) {
            Ok(_) => success += 1,
            Err(e) => {
                println!("  -> Failed to write HTML: {}", e);
                failed += 1;
            }
        }
    }

    println!("\n=== RUST FULL-SCOPE TEST RESULTS ===");
    println!("Total Success: {}", success);
    println!("Total Failed: {}", failed);
    println!("All generated HTML test outputs saved in: {:?}", output_dir);

    Ok(())
}
