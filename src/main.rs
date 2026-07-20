#![recursion_limit = "512"]

use clap::{Parser, Subcommand};
#[allow(unused_imports)]
use indexmap::IndexMap;
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

// ── AXI helpers ─────────────────────────────────────────────────────────────

/// Truncate a string to `max` chars and append a size hint when truncated.
fn truncate_str(s: &str, max: usize) -> String {
    let char_count = s.chars().count();
    if char_count <= max {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max).collect();
        format!("{}...
  ... (truncated, {} chars total)", truncated, char_count)
    }
}

/// Print a structured error to stdout (AXI §6) and exit with code 2 for usage errors.
fn axi_error(msg: &str, hint: Option<&str>) -> ! {
    let mut out = serde_json::json!({ "error": msg });
    if let Some(h) = hint {
        out["help"] = serde_json::json!(h);
    }
    println!("{}", serde_json::to_string_pretty(&out).unwrap_or_default());
    std::process::exit(2);
}

/// Get home directory path via stdlib, falling back to None.
fn dirs_or_fallback() -> Option<String> {
    std::env::var("HOME").ok().or_else(|| {
        #[cfg(windows)]
        { std::env::var("USERPROFILE").ok() }
        #[cfg(not(windows))]
        { None }
    })
}

#[derive(Subcommand)]
enum Commands {
    /// Download Chromium to ~/.slideforge/chromium/ for offline/CI installs
    Setup,
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
    /// List all available visual themes
    ListThemes,
    /// Get detailed info for a specific slide type
    SlideInfo { slide_type: String },
    /// Get slide types recommended for a specific context
    SlideTypesForContext { context: String },
    /// Generate a single slide as HTML JSON
    GenerateSlide {
        /// Slide type (e.g. hero, feature, image_headline, qr_destination)
        slide_type: String,
        /// Primary brand color in hex (required if no --tokens-file)
        #[arg(long)]
        primary_color: Option<String>,
        /// Visual theme (editorial, bold, minimal, dark, vibrant, natural)
        #[arg(long)]
        theme: Option<String>,
        /// Color preset (tonal_spot, vibrant, neutral, monochrome, expressive, fidelity)
        #[arg(long)]
        preset: Option<String>,
        /// Background style (light, dark, gradient, mesh, hero)
        #[arg(long)]
        bg_style: Option<String>,
        /// Brand archetype (educator, thought_leader, startup_pitch, brand_storyteller, data_analyst, creator)
        #[arg(long)]
        archetype: Option<String>,
        /// Platform (instagram_portrait, tiktok_vertical, etc.)
        #[arg(long)]
        platform: Option<String>,
        /// Aspect ratio (4:5, 9:16, 3:4, 1:1)
        #[arg(long)]
        aspect_ratio: Option<String>,
        /// Path to design tokens JSON (from configure-design) for session state
        #[arg(long)]
        tokens_file: Option<String>,
        /// Slide params as JSON string (e.g. '{"headline":"Hello","subheadline":"World"}')
        #[arg(long)]
        params: Option<String>,
        /// Read slide params from a JSON file
        #[arg(long)]
        params_file: Option<String>,
        /// Output file (defaults to stdout)
        #[arg(long)]
        output: Option<String>,
        /// Override token values (e.g. --override accent=#FF5500 --override secondary=#222222)
        #[arg(long = "override", value_name = "KEY=VALUE")]
        override_tokens: Vec<String>,
    },
    /// Assemble slide HTML objects into a full carousel HTML document
    RenderCarousel {
        /// Path to JSON file containing slides array (from generate-slide output)
        slides_file: String,
        /// Path to design tokens JSON (from configure-design) for CSS variables
        #[arg(long)]
        tokens_file: Option<String>,
        /// Brand name for overlay
        #[arg(long)]
        brand_name: Option<String>,
        /// Brand handle (e.g. @mybrand)
        #[arg(long)]
        brand_handle: Option<String>,
        /// Topic text for overlay
        #[arg(long)]
        topic: Option<String>,
        /// URL for overlay
        #[arg(long)]
        url: Option<String>,
        /// Comma-separated hashtags
        #[arg(long)]
        hashtags: Option<String>,
        /// Platform
        #[arg(long)]
        platform: Option<String>,
        /// Aspect ratio
        #[arg(long)]
        aspect_ratio: Option<String>,
        /// Include IG frame chrome
        #[arg(long, default_value = "true")]
        include_ig_frame: bool,
        /// Show progress indicator
        #[arg(long, default_value = "true")]
        show_progress: bool,
        /// Output file path (defaults to stdout)
        #[arg(long)]
        output: Option<String>,
    },
    /// Validate slide params before rendering
    ValidateLayout {
        slide_type: String,
        /// Slide params as JSON string
        #[arg(long)]
        params: Option<String>,
        /// Read params from JSON file
        #[arg(long)]
        params_file: Option<String>,
    },
    /// Validate carousel HTML for design issues
    ValidateDesign {
        /// Path to HTML file
        html_file: String,
    },
    /// Recommend color schemes from a primary color
    RecommendColors {
        primary: String,
        #[arg(long, default_value = "modern")]
        style: String,
        #[arg(long, default_value = "4")]
        num_schemes: u8,
    },
    /// Convert a local image file to a base64 data URI
    EmbedImage { file_path: String },
    /// Render a single slide HTML to a PNG preview
    PreviewSlide {
        /// Path to HTML file (or - for stdin)
        html_file: String,
        /// Output PNG path
        #[arg(long, default_value = "/tmp/slideforge-preview.png")]
        output: String,
    },
    /// Load the design guide and skill documentation
    SkillGuide,
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
        // ── AXI §8 + §10: no-args home view shows live state + tool identity ──
        None => {
            let bin_path = std::env::current_exe()
                .map(|p| {
                    let s = p.display().to_string();
                    if let Ok(home) = std::env::var("HOME") {
                        s.replace(&home, "~")
                    } else {
                        s
                    }
                })
                .unwrap_or_else(|_| "slideforge".to_string());
            println!("bin: {}", bin_path);
            println!("description: Generate social media carousel slides with AI-grade design systems");
            println!();
            let types = slide_registry::list_slide_types();
            println!("slides[{}]{{type,description}}:", types.len());
            for t in &types {
                if let Some(info) = slide_registry::get_slide_type_info(t) {
                    let desc = info["description"].as_str().unwrap_or("");
                    println!("  {},{}", t, truncate_str(desc, 60));
                }
            }
            println!();
            println!("help[4]:");
            println!("  Run `slideforge list-slides` to see all slide types");
            println!("  Run `slideforge list-platforms` to see export presets");
            println!("  Run `slideforge generate-slide --slide-type <type> --primary-color <hex>` to create");
            println!("  Run `slideforge mcp` to start the MCP server");
        }
        Some(Commands::Setup) => {
            println!("Downloading Chromium to ~/.slideforge/chromium/...");
            match export::download_chromium() {
                Ok(path) => {
                    let response = serde_json::json!({
                        "status": "success",
                        "binary": path.display().to_string(),
                        "message": "Chromium downloaded. Will auto-resolve on cold-start.",
                        "hint": "Set CHROME_PATH to skip auto-download."
                    });
                    println!("{}", serde_json::to_string_pretty(&response).unwrap_or_default());
                }
                Err(e) => {
                    axi_error(
                        &format!("Failed to download Chromium: {}", e),
                        Some("Manual: sudo apt install chromium-browser | brew install --cask chromium"),
                    );
                }
            }
        }
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
            export::ensure_chrome_available()?;
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
            if types.is_empty() {
                println!("slides: 0 slide types registered");
            } else {
                println!("slides[{}]{{type,description}}:", types.len());
                for t in &types {
                    if let Some(info) = slide_registry::get_slide_type_info(t) {
                        let desc = info["description"].as_str().unwrap_or("");
                        println!("  {},{}", t, truncate_str(desc, 80));
                    }
                }
            }
            println!("help[1]: Run `slideforge slide-info <type>` for details");
        }
        Some(Commands::ListPlatforms) => {
            let all = platforms::all_platforms();
            if all.is_empty() {
                println!("platforms: 0 export platforms registered");
            } else {
                println!("platforms[{}]{{name,dimensions,aspect_ratio,default_ratio}}:", all.len());
                for p in &all {
                    println!(
                        "  {},{}×{}  {},{}",
                        p.name, p.width, p.height, p.aspect_ratio, p.default_aspect_ratio
                    );
                }
            }
            println!("help[1]: Run `slideforge export --help` to see export options");
        }
        Some(Commands::ListArchetypes) => {
            let all = archetypes::all_archetypes();
            if all.is_empty() {
                println!("archetypes: 0 archetypes registered");
            } else {
                println!("archetypes[{}]{{name,description}}:", all.len());
                for a in &all {
                    println!("  {},{}", a.name, truncate_str(&a.description, 80));
                }
            }
        }
        Some(Commands::ListThemes) => {
            let themes = vec![
                ("editorial", "Clean, magazine-inspired layout with sharp edges and textured surfaces"),
                ("bold", "High-contrast, dynamic layout with strong shadows and gradient surfaces"),
                ("minimal", "Restrained layout with generous whitespace and subtle accents"),
                ("dark", "Dark-mode-first with glassmorphism and neon-adjacent accents"),
                ("vibrant", "Saturated colors with playful radii and energetic compositions"),
                ("natural", "Organic shapes, earthy palette, and hand-crafted feel"),
            ];
            println!("themes[{}]{{name,description}}:", themes.len());
            for (name, desc) in &themes {
                println!("  {},{}", name, truncate_str(desc, 80));
            }
        }
        Some(Commands::SlideInfo { slide_type }) => {
            match slide_registry::get_slide_type_info(slide_type) {
                Some(info) => {
                    let json = serde_json::to_string_pretty(&info)?;
                    println!("{}", json);
                }
                None => {
                    let valid = slide_registry::list_slide_types();
                    axi_error(
                        &format!("Unknown slide type: '{}'", slide_type),
                        Some(&format!("Valid types: {}", valid.join(", "))),
                    );
                }
            }
        }
        Some(Commands::SlideTypesForContext { context }) => {
            let types = slide_registry::get_slide_types_for_context(context);
            println!("{}", serde_json::to_string_pretty(&types)?);
        }
        Some(Commands::GenerateSlide {
            slide_type,
            primary_color,
            theme,
            preset,
            bg_style,
            archetype,
            platform,
            aspect_ratio,
            tokens_file,
            params,
            params_file,
            output,
            override_tokens,
        }) => {
            cli_generate_slide(
                slide_type,
                primary_color,
                theme,
                preset,
                bg_style,
                archetype,
                platform,
                aspect_ratio,
                tokens_file,
                params,
                params_file,
                output,
                override_tokens,
            )?;
        }
        Some(Commands::RenderCarousel {
            slides_file,
            tokens_file,
            brand_name,
            brand_handle,
            topic,
            url,
            hashtags,
            platform,
            aspect_ratio,
            include_ig_frame,
            show_progress,
            output,
        }) => {
            cli_render_carousel(
                slides_file,
                tokens_file,
                brand_name,
                brand_handle,
                topic,
                url,
                hashtags,
                platform,
                aspect_ratio,
                *include_ig_frame,
                *show_progress,
                output,
            )?;
        }
        Some(Commands::ValidateLayout {
            slide_type,
            params,
            params_file,
        }) => {
            let params_json = cli_read_params(params, params_file)?;
            let result = validate::validate_slide_spec(slide_type, &params_json);
            let response = serde_json::json!({
                "valid": result.valid,
                "errors": result.errors,
                "warnings": result.warnings,
            });
            println!("{}", serde_json::to_string_pretty(&response)?);
        }
        Some(Commands::ValidateDesign { html_file }) => {
            let html = fs::read_to_string(html_file)?;
            let report = validate::validate_design(&html);
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Some(Commands::RecommendColors {
            primary,
            style,
            num_schemes,
        }) => {
            let presets = [
                "tonal_spot",
                "vibrant",
                "neutral",
                "monochrome",
                "expressive",
                "fidelity",
            ];
            let num = (*num_schemes).min(6) as usize;
            let mut schemes = Vec::new();
            for &p in presets.iter().take(num) {
                if let Ok(tokens) =
                    design_system::derive_palette(primary, style, 16, 1.25, p, "", None, None, None)
                {
                    schemes.push(serde_json::json!({
                        "preset": p,
                        "primary": tokens.primary,
                        "primary_light": tokens.primary_light,
                        "primary_dark": tokens.primary_dark,
                        "accent": tokens.accent,
                    }));
                }
            }
            let response = serde_json::json!({
                "primary_color": primary,
                "schemes": schemes,
            });
            println!("{}", serde_json::to_string_pretty(&response)?);
        }
        Some(Commands::EmbedImage { file_path }) => {
            cli_embed_image(file_path)?;
        }
        Some(Commands::PreviewSlide { html_file, output }) => {
            export::ensure_chrome_available()?;
            let html = if html_file == "-" {
                use std::io::Read;
                let mut buf = String::new();
                std::io::stdin().read_to_string(&mut buf)?;
                buf
            } else {
                fs::read_to_string(html_file)?
            };
            let temp_html = "/tmp/slideforge-preview.html";
            let full_html = format!(
                r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><style>
body {{ margin:0; padding:0; background:#f0f0f0; display:flex; justify-content:center; align-items:center; min-height:100vh; }}
</style></head><body>{}</body></html>"#,
                html
            );
            fs::write(temp_html, full_html)?;
            match export::render_html_to_png(temp_html, output, 1.0) {
                Ok(_) => {
                    let response = serde_json::json!({
                        "png_path": output,
                        "message": format!("Preview saved to {}", output),
                    });
                    println!("{}", serde_json::to_string_pretty(&response)?);
                }
                Err(e) => {
                    axi_error(
                        &format!("Chrome render failed: {}", e),
                        Some("Ensure Chrome/Chromium is installed. Run `slideforge setup` to download."),
                    );
                }
            }
        }
        Some(Commands::SkillGuide) => {
            let content = include_str!("../DESIGN-GUIDE.md");
            println!("{}", content);
        }
        Some(Commands::TestFullScope { output_dir }) => {
            run_full_scope_test(output_dir)?;
        }
        Some(Commands::Mcp) => {
            eprintln!("Starting SlideForge MCP server (stdio)...");
            mcp_server::run_server().await?;
        }
    }

    Ok(())
}

// ── CLI helper functions (stateless equivalents of MCP tools) ────────────────

/// Read slide params from --params (JSON string) or --params-file (JSON file)
fn cli_read_params(
    params: &Option<String>,
    params_file: &Option<String>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    match params {
        Some(p) => Ok(serde_json::from_str(p)?),
        None => match params_file {
            Some(f) => {
                let content = fs::read_to_string(f)?;
                Ok(serde_json::from_str(&content)?)
            }
            None => Ok(serde_json::json!({})),
        },
    }
}

/// Load design tokens from a JSON file (produced by configure-design)
fn cli_load_tokens(
    tokens_file: &str,
) -> Result<design_system::DesignTokens, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(tokens_file)?;
    let tokens: design_system::DesignTokens = serde_json::from_str(&content)?;
    Ok(tokens)
}

/// Apply --override key=value pairs to a tokens struct.
///
/// Accepts the public color/font tokens on DesignTokens. Unknown keys are
/// surfaced as warnings rather than errors — palette experiments should not
/// block a render. Returns (applied, warnings) for the caller to echo back.
fn cli_apply_token_overrides(
    tokens: &mut design_system::DesignTokens,
    overrides: &[String],
) -> (Vec<String>, Vec<String>) {
    use serde_json::Value;
    // Serialise tokens into a JSON value so we can look up and patch any field
    // by name without a giant per-field match arm.
    let mut as_json = match serde_json::to_value(tokens.clone()) {
        Ok(v) => v,
        Err(_) => return (Vec::new(), vec!["Token override skipped: serialise failed".to_string()]),
    };

    let obj = match as_json.as_object_mut() {
        Some(o) => o,
        None => return (Vec::new(), vec!["Token override skipped: tokens is not an object".to_string()]),
    };

    let mut applied = Vec::new();
    let mut warnings = Vec::new();

    for spec in overrides {
        let Some((key, value)) = spec.split_once('=') else {
            warnings.push(format!(
                "Override '{spec}' is not 'KEY=VALUE' format — skipped"
            ));
            continue;
        };
        let key = key.trim();
        let value = value.trim();

        if !obj.contains_key(key) {
            // Suggest close matches for typo-tolerance. We try three cheap
            // heuristics in order: exact case-insensitive match, substring
            // match (either direction), then a one-edit Levenshtein
            // approximation that catches single inserts / single typos.
            let suggestion = obj
                .keys()
                .find(|k| k.eq_ignore_ascii_case(key))
                .cloned()
                .or_else(|| {
                    let k_lc = key.to_lowercase();
                    obj.keys()
                        .find(|k| k.to_lowercase().contains(&k_lc) || k_lc.contains(&k.to_lowercase()))
                        .cloned()
                })
                .or_else(|| {
                    obj.keys()
                        .find(|k| one_edit_apart(&k.to_lowercase(), &key.to_lowercase()))
                        .cloned()
                });
            match suggestion {
                Some(s) => warnings.push(format!(
                    "Override '{key}' is not a token field — did you mean '{s}'?"
                )),
                None => warnings.push(format!("Override '{key}' is not a token field — skipped")),
            }
            continue;
        }

        // Strings always overwrite as strings. Numbers/bools/types of the
        // target field could be parsed — but a CLI override should keep the
        // same intent as the existing field, so we accept stringly-typed
        // overrides (matches the natural CLI ergonomics).
        obj.insert(key.to_string(), Value::String(value.to_string()));
        applied.push(format!("{key}={value}"));
    }

    if let Ok(restored) = serde_json::from_value::<design_system::DesignTokens>(as_json) {
        *tokens = restored;
    } else {
        warnings.push("Token override could not be re-deserialised into DesignTokens — skipped".to_string());
    }

    (applied, warnings)
}

/// Cheap one-edit Levenshtein approximation: true if `a` and `b` differ by
/// exactly one insertion, deletion, or substitution. Used to power
/// typo-tolerant suggestions from `cli_apply_token_overrides`.
fn one_edit_apart(a: &str, b: &str) -> bool {
    let la = a.chars().count();
    let lb = b.chars().count();
    if (la as i32 - lb as i32).abs() > 1 {
        return false;
    }
    let av: Vec<char> = a.chars().collect();
    let bv: Vec<char> = b.chars().collect();

    if la == lb {
        let mut mismatches = 0;
        for (i, (&x, &y)) in av.iter().zip(bv.iter()).enumerate() {
            if x != y {
                mismatches += 1;
                if mismatches > 1 {
                    return false;
                }
                // One mismatched pair still allows matching tails.
                let _ = i;
            }
        }
        return mismatches == 1;
    } else if la + 1 == lb {
        // b is longer by one (single insertion in b)
        return insertion_in_longer(&av, &bv);
    } else if lb + 1 == la {
        // a is longer by one (single deletion from a = insertion in b)
        return insertion_in_longer(&bv, &av);
    }
    false
}

fn insertion_in_longer(shorter: &[char], longer: &[char]) -> bool {
    let mut i = 0;
    let mut j = 0;
    let mut skipped = false;
    while i < shorter.len() && j < longer.len() {
        if shorter[i] == longer[j] {
            i += 1;
            j += 1;
        } else if !skipped {
            skipped = true;
            j += 1; // skip the extra char in the longer string
        } else {
            return false;
        }
    }
    true
}

/// Generate a single slide — CLI equivalent of MCP generate_slide
#[allow(clippy::too_many_arguments)]
fn cli_generate_slide(
    slide_type: &str,
    primary_color: &Option<String>,
    theme: &Option<String>,
    preset: &Option<String>,
    bg_style: &Option<String>,
    archetype: &Option<String>,
    platform: &Option<String>,
    aspect_ratio: &Option<String>,
    tokens_file: &Option<String>,
    params: &Option<String>,
    params_file: &Option<String>,
    output: &Option<String>,
    override_tokens: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let slide_type = slide_type.to_lowercase().replace('-', "_");

    // Resolve tokens: either from --tokens-file or derive from --primary-color
    let (mut tokens, resolved_theme) = if let Some(tf) = tokens_file {
        let t = cli_load_tokens(tf)?;
        (t, theme.clone().unwrap_or_else(|| "editorial".to_string()))
    } else {
        let primary = primary_color
            .as_deref()
            .ok_or("Either --primary-color or --tokens-file is required")?;
        let t = theme.as_deref().unwrap_or("editorial");
        let style = match t {
            "editorial" => "editorial",
            "bold" => "bold",
            "minimal" => "modern",
            "dark" => "technical",
            "vibrant" => "rounded",
            "natural" => "warm",
            _ => "modern",
        };
        let p = preset.as_deref().unwrap_or("tonal_spot");
        let plt = platform.as_deref().unwrap_or("instagram_portrait");
        let canvas = platforms::resolve_canvas(plt, aspect_ratio.as_deref())?;
        let tokens = design_system::derive_palette_with_canvas(
            primary,
            style,
            16,
            1.25,
            p,
            t,
            None,
            None,
            None,
            canvas.width,
            canvas.height,
        )?;
        (tokens, t.to_string())
    };

    // Apply --override KEY=VALUE patches to the resolved tokens (single-value
    // patches without rerunning palette derivation). Unknown keys are
    // collected and reported as warnings after the render.
    let (overrides_applied, overrides_warnings) =
        cli_apply_token_overrides(&mut tokens, override_tokens);

    let bg = bg_style.as_deref().unwrap_or("light");
    let arch = archetype.as_deref().unwrap_or("educator");

    let params_json = cli_read_params(params, params_file)?;

    // Pre-flight validation — abort if required params are missing
    let validation = validate::validate_slide_spec(&slide_type, &params_json);

    if !validation.errors.is_empty() {
        let response = serde_json::json!({
            "success": false,
            "slide_type": slide_type,
            "validation": {
                "errors": validation.errors,
                "warnings": validation.warnings,
            },
            "hint": format!("Run 'slideforge slide-info {}' to see required params.", slide_type),
        });
        eprintln!("{}", serde_json::to_string_pretty(&response)?);
        std::process::exit(1);
    }

    let result = components::dispatch_slide(
        &slide_type,
        &tokens,
        &params_json,
        bg,
        &resolved_theme,
        arch,
    )?;

    // Enrich with slide_type + validation warnings (errors already blocked above)
    let mut enriched = result;
    if let Some(obj) = enriched.as_object_mut() {
        obj.insert("slide_type".to_string(), serde_json::json!(slide_type));
        if !validation.warnings.is_empty() {
            obj.insert(
                "validation".to_string(),
                serde_json::json!({
                    "warnings": validation.warnings,
                }),
            );
        }
        if !overrides_applied.is_empty() || !overrides_warnings.is_empty() {
            obj.insert(
                "token_overrides".to_string(),
                serde_json::json!({
                    "applied": overrides_applied,
                    "warnings": overrides_warnings,
                }),
            );
        }
    }

    let json = serde_json::to_string_pretty(&enriched)?;
    match output {
        Some(path) => {
            fs::write(path, &json)?;
            eprintln!("Slide saved to {}", path);
        }
        None => println!("{}", json),
    }
    Ok(())
}

/// Render a carousel — CLI equivalent of MCP render_carousel
#[allow(clippy::too_many_arguments)]
fn cli_render_carousel(
    slides_file: &str,
    tokens_file: &Option<String>,
    brand_name: &Option<String>,
    brand_handle: &Option<String>,
    topic: &Option<String>,
    url: &Option<String>,
    hashtags: &Option<String>,
    platform: &Option<String>,
    aspect_ratio: &Option<String>,
    include_ig_frame: bool,
    show_progress: bool,
    output: &Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Load slides array from file
    let slides_content = fs::read_to_string(slides_file)?;
    let slides_json: serde_json::Value = serde_json::from_str(&slides_content)?;

    // Accept either an array of slide specs or a single slide spec
    let slides_arr: Vec<slides::SlideSpec> = match &slides_json {
        serde_json::Value::Array(_) => serde_json::from_value(slides_json.clone())?,
        serde_json::Value::Object(obj) => {
            if obj.contains_key("slides") {
                let arr = obj.get("slides").cloned().unwrap_or(serde_json::json!([]));
                serde_json::from_value(arr)?
            } else {
                // Single slide spec — wrap in array
                let single: slides::SlideSpec = serde_json::from_value(slides_json.clone())?;
                vec![single]
            }
        }
        _ => return Err("slides_file must contain a JSON array or object".into()),
    };

    // Load tokens for CSS variables
    let (css_vars, google_fonts, heading_font, body_font) = if let Some(tf) = tokens_file {
        let tokens = cli_load_tokens(tf)?;
        (
            tokens.to_css_variables(),
            tokens.google_fonts_url.clone(),
            tokens.heading_font.clone(),
            tokens.body_font.clone(),
        )
    } else {
        // Minimal defaults
        (
            r#"--primary: #6366F1; --primary-light: #A5B4FC; --primary-dark: #4338CA; --accent: #EC4899; --secondary: #10B981; --tertiary: #F59E0B; --surface-light: #F8FAFC; --surface-dark: #0F172A; --text-primary: #0F172A; --text-secondary: #475569; --text-on-dark: #F1F5F9; --text-on-dark-secondary: #94A3B8; --border-light: #CBD5E1; --border-dark: #334155;"#.to_string(),
            "https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;500;600;700;800&display=swap".to_string(),
            "Plus Jakarta Sans".to_string(),
            "Plus Jakarta Sans".to_string(),
        )
    };

    let plt = platform.as_deref().unwrap_or("instagram_portrait");
    let ar = aspect_ratio.as_deref();
    let canvas = platforms::resolve_canvas(plt, ar)?;

    let spec = slides::CarouselSpec {
        slides: slides_arr,
        css_variables: css_vars,
        google_fonts_url: google_fonts,
        heading_font,
        body_font,
        brand_name: brand_name.clone().unwrap_or_else(|| "Brand".to_string()),
        brand_handle: brand_handle.clone().unwrap_or_else(|| "@brand".to_string()),
        topic: topic.clone().unwrap_or_default(),
        url: url.clone().unwrap_or_default(),
        hashtags: hashtags
            .as_deref()
            .map(|s| s.split(',').map(|t| t.trim().to_string()).collect())
            .unwrap_or_default(),
        show_progress,
        visual_theme: "editorial".to_string(),
        include_ig_frame,
        platform: canvas.platform.clone(),
        aspect_ratio: canvas.aspect_ratio.clone(),
        canvas_width: canvas.width,
        canvas_height: canvas.height,
    };

    let html = slides::render_carousel_html(&spec);

    match output {
        Some(path) => {
            fs::write(path, &html)?;
            eprintln!("Carousel HTML saved to {}", path);
        }
        None => println!("{}", html),
    }
    Ok(())
}

/// Convert a local image file to a base64 data URI — CLI equivalent of MCP embed_local_image
fn cli_embed_image(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::path::Path;

    let p = Path::new(file_path);
    if !p.exists() {
        axi_error(
            &format!("File not found: {}", file_path),
            None,
        );
    }

    let ext = p
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();
    let mime = match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        _ => {
            axi_error(
                &format!("Unsupported image extension '{}'", ext),
                Some("Supported: png, jpg/jpeg, gif, webp, svg"),
            );
        }
    };

    let bytes = fs::read(p)?;
    let size_kb = bytes.len() / 1024;

    // Base64 encode
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut b64 = String::with_capacity((bytes.len() + 2) / 3 * 4);
    for chunk in bytes.chunks(3) {
        let b = match chunk.len() {
            3 => ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | (chunk[2] as u32),
            2 => ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8),
            1 => (chunk[0] as u32) << 16,
            _ => 0,
        };
        b64.push(CHARS[((b >> 18) & 0x3F) as usize] as char);
        b64.push(CHARS[((b >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            b64.push(CHARS[((b >> 6) & 0x3F) as usize] as char);
        } else {
            b64.push('=');
        }
        if chunk.len() > 2 {
            b64.push(CHARS[(b & 0x3F) as usize] as char);
        } else {
            b64.push('=');
        }
    }

    let data_uri = if mime == "image/svg+xml" {
        let svg_text = String::from_utf8_lossy(&bytes);
        let encoded = svg_text
            .replace('#', "%23")
            .replace('<', "%3C")
            .replace('>', "%3E")
            .replace('"', "'");
        format!("data:image/svg+xml;utf8,{}", encoded)
    } else {
        format!("data:{};base64,{}", mime, b64)
    };

    let warning = if size_kb > 2048 {
        Some(format!(
            "Image is {}KB — consider resizing to <500KB.",
            size_kb
        ))
    } else if size_kb > 500 {
        Some(format!(
            "Image is {}KB — consider resizing for optimal export.",
            size_kb
        ))
    } else {
        None
    };

    let response = serde_json::json!({
        "data_uri": data_uri,
        "mime_type": mime,
        "size_bytes": bytes.len(),
        "size_kb": size_kb,
        "warning": warning,
    });
    println!("{}", serde_json::to_string_pretty(&response)?);
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
    ]
    .iter()
    .cloned()
    .collect();

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
        let allowed_ratios = platform_ratio_overrides
            .get(platform)
            .unwrap_or(&default_ratios);
        let aspect_ratio = allowed_ratios[idx % allowed_ratios.len()];
        let color = brand_colors[idx % brand_colors.len()];
        let theme = themes[idx % themes.len()];
        let carousel_id = idx + 1;
        let canvas = platforms::resolve_canvas(platform, Some(aspect_ratio))?;
        // Token generation always uses 4:5 composition (420×525)
        let base_width = 420;
        let base_height = 525;

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

        // 1. Derive tokens at 420px base (vectoric scaling handles upscaling)
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
            base_width,
            base_height,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::design_system::DesignTokens;

    fn minimal_tokens() -> DesignTokens {
        DesignTokens {
            primary: "#4f46e5".to_string(),
            primary_light: "#a5b4fc".to_string(),
            primary_dark: "#3730a3".to_string(),
            surface_light: "#ffffff".to_string(),
            surface_dark: "#101014".to_string(),
            text_primary: "#18181b".to_string(),
            text_secondary: "#52525b".to_string(),
            text_on_dark: "#f8fafc".to_string(),
            text_on_dark_secondary: "#cbd5e1".to_string(),
            border_light: "#e4e4e7".to_string(),
            border_dark: "#27272a".to_string(),
            accent: "#ec4899".to_string(),
            secondary: "#f59e0b".to_string(),
            tertiary: "#10b981".to_string(),
            gradient: "linear-gradient(135deg,#4f46e5,#ec4899)".to_string(),
            temperature: "cool".to_string(),
            heading_font: "Inter".to_string(),
            body_font: "Inter".to_string(),
            google_fonts_url: "https://fonts.googleapis.com/css2?family=Inter".to_string(),
            type_scale: IndexMap::new(),
            spacing: IndexMap::new(),
            contrast_report: IndexMap::new(),
            shadows: IndexMap::new(),
            radii: IndexMap::new(),
            gradients: IndexMap::new(),
            textures: IndexMap::new(),
            glass: serde_json::json!({}),
        }
    }

    #[test]
    fn test_cli_token_override_applies_known_fields() {
        let mut tokens = minimal_tokens();
        let (applied, warnings) =
            cli_apply_token_overrides(&mut tokens, &["primary=#FF5500".to_string()]);

        assert_eq!(applied, vec!["primary=#FF5500".to_string()]);
        assert!(warnings.is_empty(), "no warnings expected, got {warnings:?}");
        assert_eq!(tokens.primary, "#FF5500");
    }

    #[test]
    fn test_cli_token_override_warns_on_unknown_key_with_suggestion() {
        let mut tokens = minimal_tokens();
        let (applied, warnings) = cli_apply_token_overrides(
            &mut tokens,
            &["acccent=#123456".to_string()], // typo: acccent
        );

        assert!(applied.is_empty(), "typo key should NOT be applied");
        assert_eq!(warnings.len(), 1);
        assert!(
            warnings[0].contains("accent"),
            "suggestion should mention 'accent', got: {warnings:?}"
        );
    }

    #[test]
    fn test_cli_token_override_ignores_malformed_spec() {
        let mut tokens = minimal_tokens();
        let (applied, warnings) =
            cli_apply_token_overrides(&mut tokens, &["notakeyvalue".to_string()]);

        assert!(applied.is_empty());
        assert!(
            warnings[0].contains("KEY=VALUE"),
            "warning names the expected format"
        );
    }

    #[test]
    fn test_cli_token_override_preserves_other_fields() {
        let mut tokens = minimal_tokens();
        let _ = cli_apply_token_overrides(&mut tokens, &["accent=#000000".to_string()]);

        assert_eq!(tokens.primary, "#4f46e5");
        assert_eq!(tokens.accent, "#000000");
        assert_eq!(tokens.heading_font, "Inter");
    }

    #[test]
    fn test_one_edit_apart_detects_typos() {
        // ponytail: single-insertion, single-deletion, single-substitution.
        // True Levenshtein (with transposition) not implemented — Damerau is
        // not worth the complexity for a CLI typo hint; substring / case-
        // insensitive match catches the rest in practice.
        assert!(one_edit_apart("accent", "acccent"), "extra insertion");
        assert!(one_edit_apart("accent", "accen"), "single deletion");
        assert!(one_edit_apart("accent", "accint"), "single substitution");
        assert!(!one_edit_apart("accent", "primary"), "unrelated keys");
        assert!(!one_edit_apart("accent", "verydifferentkey"), "far apart");
    }
}
