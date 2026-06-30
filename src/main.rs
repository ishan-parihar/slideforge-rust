#![recursion_limit = "512"]

use clap::{Parser, Subcommand};
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng, rngs::StdRng};
use serde_json::json;
use std::fs;
use std::path::Path;

mod archetypes;
mod blocks;
mod components;
mod dataviz;
mod design_system;
mod effects;
mod export;
mod layouts;
mod mcp_server;
mod platforms;
mod slide_registry;
mod slides;
mod validate;

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
        /// Custom aspect ratio override
        #[arg(long)]
        aspect_ratio: Option<String>,
    },
    /// List all available slide types with descriptions
    ListSlides,
    /// List all available export platform presets
    ListPlatforms,
    /// List all available brand archetypes
    ListArchetypes,
    /// Run an exhaustive full-scope test generating 24 carousels covering all archetypes, themes, and slide types
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
        Some(Commands::ConfigureDesign {
            primary,
            style,
            preset,
            output,
        }) => {
            println!("Generating design system for {}...", primary);
            let tokens = design_system::derive_palette(
                primary, style, 16, 1.25, preset, "", None, None, None,
            )?;
            let json = serde_json::to_string_pretty(&tokens)?;
            fs::write(output, json)?;
            println!("Design tokens saved to {}", output);
        }
        Some(Commands::Export {
            html,
            output_dir,
            slides,
            preset,
            aspect_ratio,
        }) => {
            let canvas = platforms::resolve_canvas(preset, aspect_ratio.as_deref())?;
            println!(
                "Exporting {} slides from {} → {} at {}×{} (platform: {}, aspect ratio: {})...",
                slides,
                html,
                output_dir,
                canvas.width,
                canvas.height,
                canvas.platform,
                canvas.aspect_ratio
            );
            let paths =
                export::export_slides(html, output_dir, *slides, canvas.width, canvas.height)
                    .await?;
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
                println!(
                    "  {:<25} {}×{} (aspect ratio: {}, default aspect ratio: {}, allowed ratios: {})",
                    p.name,
                    p.width,
                    p.height,
                    p.aspect_ratio,
                    p.default_aspect_ratio,
                    p.allowed_aspect_ratios.join(", ")
                );
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
            eprintln!("Starting SlideForge MCP server (stdio)...");
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
        "hero",
        "feature",
        "list",
        "quote",
        "cta",
        "comparison",
        "stat_row",
        "timeline",
        "callout",
        "split_features",
        "grid_cards",
        "headline_subheadline",
        "definition",
        "text_block",
        "metric_card",
        "chart",
        "progress_rings",
        "comparison_bars",
        "metric_grid",
        "funnel_chart",
        "table",
        "metric_sparkline",
        "column_chart",
        "text_columns",
        "qr_destination",
    ];

    let archetypes_list = vec![
        "educator",
        "thought_leader",
        "startup_pitch",
        "brand_storyteller",
        "data_analyst",
        "creator",
    ];

    let platforms_list = vec![
        "instagram_portrait",
        "instagram_square",
        "instagram_story",
        "tiktok_vertical",
        "linkedin_landscape",
    ];

    let platform_ratio_overrides: std::collections::HashMap<&str, Vec<&str>> = [
        ("instagram_portrait", vec!["4:5", "3:4", "1:1"]),
        ("instagram_square", vec!["1:1", "4:5"]),
        ("instagram_story", vec!["9:16", "3:4"]),
        ("tiktok_vertical", vec!["9:16"]),
        ("linkedin_landscape", vec!["4:5", "1:1"]),
    ].iter().cloned().collect();

    let default_ratios = vec!["4:5"];

    let brand_colors = vec![
        "#6366F1", "#EF4444", "#10B981", "#F59E0B", "#8B5CF6", "#EC4899", "#06B6D4", "#84CC16",
    ];

    let themes = vec!["editorial", "bold", "minimal", "dark", "vibrant", "natural"];

    let mut rng = StdRng::seed_from_u64(42);
    let mut success = 0;
    let mut failed = 0;
    let total_carousels = 24; // Updated to include qr_destination

    for idx in 0..total_carousels {
        let archetype = archetypes_list[idx % archetypes_list.len()];
        let platform = platforms_list[idx % platforms_list.len()];
        let allowed_ratios = platform_ratio_overrides.get(platform).unwrap_or(&default_ratios);
        let aspect_ratio = allowed_ratios[idx % allowed_ratios.len()];
        let color = brand_colors[idx % brand_colors.len()];
        let theme = themes[idx % themes.len()];
        let carousel_id = idx + 1;
        let canvas = platforms::resolve_canvas(platform, Some(aspect_ratio))?;

        // Choose 4 random slide types
        let mut chosen_types = slide_types.clone();
        chosen_types.shuffle(&mut rng);
        let carousel_types = &chosen_types[0..4];

        let topics_list = vec![
            "AI Analytics",
            "Cloud Orchestration",
            "Design System Convergence",
            "B2B SaaS Growth",
            "Performance Engine Scaling",
            "Cross-Platform Compilers",
            "Responsive Web Layouts",
        ];
        let topic = topics_list[idx % topics_list.len()];

        println!(
            "[{}/{}] Testing {}, platform: {}, ratio: {}, theme: {} with slides: {:?}",
            carousel_id, total_carousels, archetype, platform, aspect_ratio, theme, carousel_types
        );

        // 1. Derive tokens with canvas-aware scaling
        let tokens = design_system::derive_palette_with_canvas(
            color,
            "modern",
            16,
            1.25,
            "tonal_spot",
            theme,
            None,
            None,
            None,
            canvas.width,
            canvas.height,
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
            // Dynamically assign background styles to test all types: light, dark, gradient, mesh
            let bg_options = ["light", "dark", "gradient", "mesh"];
            let rand_bg = bg_options[rng.r#gen::<usize>() % bg_options.len()];

            // Randomize variants to test multiple layouts
            let hero_variants = ["centered", "left-aligned", "split"];
            let feature_variants = ["stacked", "icon-left", "icon-right", "minimal"];
            let list_variants = ["bullet", "numbered", "checklist", "icon-list", "two-column"];
            let chart_types = ["bar", "pie", "donut", "line", "area", "scatter"];

            let params = match *st {
                "hero" => json!({
                    "headline": format!("Introducing AI Analytics for {}", archetype),
                    "subheadline": format!("Powering content workflows at scale via {} platform.", platform),
                    "badge": "NEW",
                    "variant": hero_variants[rng.r#gen::<usize>() % hero_variants.len()],
                    "bg_style": rand_bg
                }),
                "cta" => json!({
                    "headline": format!("Ready to Scale your {}?", topic),
                    "button_text": "Launch Free Trial",
                    "button_url": "https://slideforge.dev",
                    "subtext": "No credit card required • Instant setup",
                    "bg_style": rand_bg
                }),
                "feature" => json!({
                    "icon": "⚡",
                    "title": "Unmatched Performance",
                    "description": "Engineered with optimized native system kernels.",
                    "number": "12x",
                    "variant": feature_variants[rng.r#gen::<usize>() % feature_variants.len()],
                    "bg_style": rand_bg
                }),
                "list" => json!({
                    "title": "Platform Core Offerings",
                    "items": [
                        {"title": "Automated Workflows", "description": "Saves 10+ hours per week of manual labor"},
                        {"title": "Visual Insights", "description": "Interactive data visualization on any screen size"},
                        {"title": "Multi-channel Export", "description": "Deploy to LinkedIn, TikTok, and Instagram instantly"}
                    ],
                    "variant": list_variants[rng.r#gen::<usize>() % list_variants.len()],
                    "bg_style": rand_bg
                }),
                "quote" => json!({
                    "quote": "SlideForge has fundamentally changed our design turnaround. Parity is absolute.",
                    "author": "Marcus Aurelius",
                    "role": "Lead Architect",
                    "bg_style": rand_bg
                }),
                "comparison" => json!({
                    "title": "Traditional vs Modern Workflows",
                    "left_label": "Legacy Approach",
                    "right_label": "SlideForge Engine",
                    "left_items": ["Manual assets stitching", "Broken layouts and centering", "No visual theme consistency"],
                    "right_items": ["Atomic layout validation", "Stunning responsive templates", "Guaranteed design system parity"],
                    "bg_style": rand_bg
                }),
                "stat_row" => json!({
                    "title": "Verified Performance Growth",
                    "stats": [
                        {"value": "99.99%", "label": "API Reliability"},
                        {"value": "12.8M", "label": "Slides Generated"},
                        {"value": "4.8/5", "label": "Customer Rating"}
                    ],
                    "bg_style": rand_bg
                }),
                "timeline" => json!({
                    "title": "Rapid Integration Roadmap",
                    "steps": [
                        {"title": "1. Scoping", "description": "Establish theme parameters"},
                        {"title": "2. Layout Audit", "description": "Scan and trace parity gaps"},
                        {"title": "3. Deployment", "description": "Automated slide generation"}
                    ],
                    "bg_style": rand_bg
                }),
                "callout" => json!({
                    "title": "System Alert",
                    "text": "Enable duotone cool filters on dark slides to maximize aesthetic impact.",
                    "icon": "🛡️",
                    "variant": "success",
                    "bg_style": rand_bg
                }),
                "split_features" => json!({
                    "title": "Platform Benefits",
                    "features": [
                        {"title": "Sub-100ms Edge Latency", "description": "Distributed content networks serving assets globally."},
                        {"title": "Intelligent Centering", "description": "Automatically validates flexbox align and vertical centering parameters."}
                    ],
                    "bg_style": rand_bg
                }),
                "grid_cards" => json!({
                    "title": "Advanced Capabilities",
                    "cards": [
                        {"icon": "🤖", "title": "Generative Agents", "description": "Autonomous subagents executing complex layout implementations."},
                        {"icon": "📈", "title": "Real-time Metrics", "description": "Live reporting pipelines directly fed from Excel sheets."},
                        {"icon": "🎨", "title": "Aesthetic Presets", "description": "Polished responsive cards mapped dynamically to typography scale."}
                    ],
                    "bg_style": rand_bg
                }),
                "headline_subheadline" => json!({
                    "headline": format!("Announcing the Next Generation of {}", topic),
                    "subheadline": "Unifying design system definitions across Rust and Python engines.",
                    "bg_style": rand_bg
                }),
                "definition" => json!({
                    "term": "Design System Parity",
                    "definition": "The systematic convergence of visual styling variables, layout dimensions, templates, and theme configuration sets between two distinct compiler runtimes.",
                    "context": "Software Porting Architecture",
                    "bg_style": rand_bg
                }),
                "text_block" => json!({
                    "title": "Principles of Aesthetic Layouts",
                    "body": "Aesthetics are a primary developer criteria. Using generic colors or raw gray backgrounds results in amateur layouts.\nAlways leverage duotone gradients, fine noise textures, and micro-interactions.",
                    "bg_style": rand_bg
                }),
                "metric_card" => json!({
                    "value": "$48.2K",
                    "label": "Average MRR Growth",
                    "trend": "↑ 42% monthly increase",
                    "context": "Audited by SlideForge financial tracking core",
                    "bg_style": rand_bg
                }),
                "progress_rings" => json!({
                    "title": "Completion Metrics",
                    "items": [
                        {"label": "Tasks Done", "value": 85.0, "color": "#10B981"},
                        {"label": "Code Quality", "value": 72.0, "color": "#6366F1"},
                        {"label": "Test Coverage", "value": 91.0, "color": "#EF4444"}
                    ],
                    "bg_style": rand_bg
                }),
                "comparison_bars" => json!({
                    "title": "Performance Comparison",
                    "metrics": [
                        {"name": "Latency", "left_value": 120.0, "right_value": 45.0, "left_label": "Legacy", "right_label": "New"},
                        {"name": "Throughput", "left_value": 800.0, "right_value": 2400.0, "left_label": "Legacy", "right_label": "New"}
                    ],
                    "bg_style": rand_bg
                }),
                "metric_grid" => json!({
                    "title": "Key Performance Indicators",
                    "metrics": [
                        {"value": "12.8M", "label": "Slides Generated", "trend": "+24%"},
                        {"value": "99.99%", "label": "API Uptime", "trend": "+0.01%"},
                        {"value": "4.8/5", "label": "User Rating", "trend": "+0.3"},
                        {"value": "180ms", "label": "Avg Render Time", "trend": "-15ms"}
                    ],
                    "bg_style": rand_bg
                }),
                "funnel_chart" => json!({
                    "title": "Conversion Funnel",
                    "steps": [
                        {"label": "Visitors", "value": 10000},
                        {"label": "Signups", "value": 3500},
                        {"label": "Activated", "value": 1800},
                        {"label": "Subscribed", "value": 890}
                    ],
                    "bg_style": rand_bg
                }),
                "table" => json!({
                    "title": "Feature Comparison",
                    "columns": ["Feature", "Basic", "Pro", "Enterprise"],
                    "rows": [
                        ["Slides/mo", "50", "500", "Unlimited"],
                        ["Export", "PNG", "PNG + SVG", "All Formats"],
                        ["Support", "Community", "Email", "Priority"]
                    ],
                    "bg_style": rand_bg
                }),
                "metric_sparkline" => json!({
                    "value": "89.2%",
                    "label": "System Health",
                    "data": [65.0, 72.0, 78.0, 85.0, 82.0, 88.0, 91.0, 89.0],
                    "context": "Last 8 measurements",
                    "trend": "↑ Stable",
                    "bg_style": rand_bg
                }),
                "column_chart" => json!({
                    "title": "Monthly Active Users",
                    "caption": "Growth trajectory across product launch phases",
                    "data": [
                        {"label": "Jan", "value": 1200.0},
                        {"label": "Feb", "value": 1850.0},
                        {"label": "Mar", "value": 2700.0},
                        {"label": "Apr", "value": 3100.0}
                    ],
                    "bg_style": rand_bg
                }),
                "text_columns" => json!({
                    "title": "Core Architecture Principles",
                    "columns": [
                        {"heading": "Performance", "text": "Sub-100ms render times via native compilation."},
                        {"heading": "Parity", "text": "Pixel-perfect alignment with Python reference."},
                        {"heading": "Scalability", "text": "Handles thousands of slides without degradation."}
                    ],
                    "bg_style": rand_bg
                }),
                "qr_destination" => json!({
                    "destination_url": "https://nexusai.io/blog/agentic-slide-workflows",
                    "heading": "Read the full workflow",
                    "caption": "A practical guide to turning carousel attention into owned traffic.",
                    "cta_text": "Scan to read",
                    "short_url": "nexusai.io/guide",
                    "incentive_text": "Includes templates and examples.",
                    "variant": "full-conversion",
                    "bg_style": rand_bg
                }),
                "chart" => {
                    let chart_type = chart_types[rng.r#gen::<usize>() % chart_types.len()];
                    let chart_data = if chart_type == "scatter" {
                        json!([
                            {"x": 10.0, "y": 25.0, "size": 8.0, "label": "Alpha"},
                            {"x": 20.0, "y": 45.0, "size": 12.0, "label": "Beta"},
                            {"x": 40.0, "y": 68.0, "size": 15.0, "label": "Gamma"},
                            {"x": 80.0, "y": 95.0, "size": 22.0, "label": "Delta"}
                        ])
                    } else {
                        json!([
                            {"label": "Q1", "value": 24.0},
                            {"label": "Q2", "value": 38.0},
                            {"label": "Q3", "value": 55.0},
                            {"label": "Q4", "value": 89.0}
                        ])
                    };
                    json!({
                        "chart_type": chart_type,
                        "title": format!("Annual Revenue Growth ({})", chart_type),
                        "caption": "Projected growth rates across all corporate verticals.",
                        "data": chart_data,
                        "bg_style": rand_bg
                    })
                }
                _ => json!({}),
            };

            // Inject background_image 25% of the time (matching Python test)
            let mut params = params;
            if rng.r#gen::<f32>() < 0.25 {
                let test_images = [
                    "https://images.unsplash.com/photo-1451187580459-43490279c0fa",
                    "https://images.unsplash.com/photo-1518770660439-4636190af475",
                    "https://images.unsplash.com/photo-1506748686214-e9df14d4d9d0",
                ];
                let img_url = *test_images.choose(&mut rng).unwrap();
                let img_opacity: f32 = 0.15 + rng.r#gen::<f32>() * 0.30;
                if let Some(obj) = params.as_object_mut() {
                    obj.insert("background_image".to_string(), json!(img_url));
                    obj.insert("image_opacity".to_string(), json!(img_opacity));
                }
            }

            let bg_style = params["bg_style"].as_str().unwrap_or("light");

            let slide_val =
                components::dispatch_slide(st, &tokens, &params, bg_style, theme, archetype);
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
            hashtags: vec![
                "#nexusai".to_string(),
                format!("#{}", archetype),
                format!("#{}", theme),
            ],
            show_progress: true,
            visual_theme: theme.to_string(),
            include_ig_frame: true,
            platform: canvas.platform,
            aspect_ratio: canvas.aspect_ratio,
            canvas_width: canvas.width,
            canvas_height: canvas.height,
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
