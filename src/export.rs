use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;
use headless_chrome::protocol::cdp::Page::Viewport;
use headless_chrome::{Browser, FetcherOptions, LaunchOptions};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

/// Directory where `slideforge setup` downloads Chromium.
pub fn slideforge_chromium_dir() -> PathBuf {
    dirs()
        .home_dir()
        .join(".slideforge")
        .join("chromium")
}

const CHROME_INSTALL_HINT: &str = "Chromium/Chrome is not installed or not on PATH.\n\
    Install options:\n\
      1. Auto-download:   slideforge setup\n\
      2. Ubuntu/Debian:   sudo apt install chromium-browser\n\
      3. macOS:           brew install --cask chromium\n\
      4. Windows:         https://www.chromium.org/getting-involved/download-chromium/\n\
    Or set CHROME_PATH env var to your Chrome/Chromium binary.";

/// Helper to get the user's home directory.
fn dirs() -> HomeDir {
    HomeDir
}

struct HomeDir;
impl HomeDir {
    fn home_dir(&self) -> PathBuf {
        std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."))
    }
}

/// Chrome flags that reduce memory footprint by ~60-75%.
/// - single-process: eliminates separate GPU/utility/network processes (~300 MB)
/// - disable-gpu: no SwiftShader GPU process (~50 MB)
/// - disable-dev-shm-usage: use /tmp not /dev/shm (container-safe)
/// - headless=new: smaller headless mode
const CHROME_ARGS: &[&str] = &[
    "--headless=new",
    "--no-sandbox",
    "--disable-gpu",
    "--disable-dev-shm-usage",
    "--disable-extensions",
    "--disable-background-networking",
    "--disable-background-timer-throttling",
    "--disable-backgrounding-occluded-windows",
    "--disable-breakpad",
    "--disable-client-side-phishing-detection",
    "--disable-component-extensions-with-background-pages",
    "--disable-default-apps",
    "--disable-hang-monitor",
    "--disable-ipc-flooding-protection",
    "--disable-popup-blocking",
    "--disable-prompt-on-repost",
    "--disable-renderer-backgrounding",
    "--disable-sync",
    "--disable-site-isolation-trials",
    "--force-color-profile=srgb",
    "--metrics-recording-only",
    "--no-first-run",
    "--enable-automation",
    "--password-store=basic",
    "--use-mock-keychain",
    "--single-process",
];

/// Build LaunchOptions with memory-optimized Chrome flags.
fn optimized_launch_options() -> Result<headless_chrome::LaunchOptions<'static>, String> {
    let args: Vec<&OsStr> = CHROME_ARGS.iter().map(|s| s.as_ref()).collect();
    LaunchOptions::default_builder()
        .headless(true)
        .args(args)
        .build()
        .map_err(|e| format!("Failed to configure headless browser: {}", e))
}

/// Check if the `slideforge setup` pre-downloaded Chrome exists.
pub fn slideforge_chrome_path() -> Option<PathBuf> {
    let base = slideforge_chromium_dir();
    // headless_chrome fetcher stores downloads under `linux-{rev}/chrome-linux/chrome`
    if base.exists() {
        for entry in fs::read_dir(&base).ok()? {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_dir() {
                // Check linux-{rev}/chrome-linux/chrome (extracted zip structure)
                let chrome_bin = path.join("chrome-linux").join("chrome");
                if chrome_bin.exists() {
                    return Some(chrome_bin);
                }
                // Fallback: check linux-{rev}/chrome (flat structure)
                let chrome_bin_flat = path.join("chrome");
                if chrome_bin_flat.exists() {
                    return Some(chrome_bin_flat);
                }
            }
        }
    }
    None
}

/// Verify that a headless Chrome instance can be launched.
/// Tries in order: CHROME_PATH env var → pre-downloaded ~/.slideforge/chromium/ → auto-download via fetch → error with instructions.
pub fn ensure_chrome_available() -> Result<(), String> {
    // 1. Try pre-downloaded path first (from `slideforge setup`)
    if let Some(path) = slideforge_chrome_path() {
        let ops = optimized_launch_options_with_path(&path)?;
        if Browser::new(ops).is_ok() {
            return Ok(());
        }
    }
    // 2. Try system Chrome / auto-download (headless_chrome `fetch` feature handles this)
    let ops = optimized_launch_options()?;
    Browser::new(ops).map_err(|_| CHROME_INSTALL_HINT.to_string())?;
    Ok(())
}

/// Build LaunchOptions with a specific Chrome path and memory-optimized flags.
fn optimized_launch_options_with_path(path: &Path) -> Result<headless_chrome::LaunchOptions<'static>, String> {
    let args: Vec<&OsStr> = CHROME_ARGS.iter().map(|s| s.as_ref()).collect();
    LaunchOptions::default_builder()
        .headless(true)
        .args(args)
        .path(Some(path.to_path_buf()))
        .build()
        .map_err(|e| format!("Failed to configure headless browser: {}", e))
}

/// Download Chromium to ~/.slideforge/chromium/ for scripted/CI installs.
/// Uses headless_chrome's built-in fetcher with a custom install directory.
pub fn download_chromium() -> Result<PathBuf, String> {
    let install_dir = slideforge_chromium_dir();
    fs::create_dir_all(&install_dir).map_err(|e| format!("Failed to create directory {}: {}", install_dir.display(), e))?;

    // Configure the fetcher to download ONLY to our custom directory
    // allow_standard_dirs(false) prevents the fetcher from finding system Chrome
    // and forces a fresh download to install_dir
    let fetcher_options = FetcherOptions::default()
        .with_install_dir(Some(install_dir.clone()))
        .with_allow_download(true)
        .with_allow_standard_dirs(false);

    // Use Browser::new with custom fetcher_options to trigger download
    let args: Vec<&OsStr> = CHROME_ARGS.iter().map(|s| s.as_ref()).collect();
    let ops = LaunchOptions::default_builder()
        .headless(true)
        .args(args)
        .fetcher_options(fetcher_options)
        .build()
        .map_err(|e| format!("Failed to configure download: {}", e))?;

    // Browser::new triggers the fetch if chrome isn't found
    let _browser = Browser::new(ops).map_err(|e| format!("Failed to download Chromium: {}", e))?;
    drop(_browser);

    // Find the downloaded chrome binary
    let chrome_path = slideforge_chrome_path()
        .ok_or_else(|| format!("Chromium downloaded but not found in {}", install_dir.display()))?;

    Ok(chrome_path)
}

/// Render a single HTML file to a PNG. Used by the `preview_slide` MCP tool
/// for quick single-slide previews without the full carousel export dance.
pub fn render_html_to_png(html_path: &str, output_path: &str, _scale: f32) -> Result<(), String> {
    let abs_html_path = fs::canonicalize(html_path)
        .map_err(|e| format!("Could not canonicalize HTML path: {}", e))?;
    let file_url = format!("file://{}", abs_html_path.to_string_lossy());

    let ops = optimized_launch_options()?;
    let browser = Browser::new(ops).map_err(|e| format!("{} ({})", CHROME_INSTALL_HINT, e))?;
    let tab = browser.new_tab().map_err(|e| format!("Failed to open browser tab: {}", e))?;

    use headless_chrome::types::Bounds;
    // Use a generous viewport; the slide centers itself via body flexbox
    tab.set_bounds(Bounds::Normal {
        left: None,
        top: None,
        width: Some(800.0),
        height: Some(1000.0),
    })
    .map_err(|e| e.to_string())?;

    tab.navigate_to(&file_url).map_err(|e| e.to_string())?;
    tab.wait_until_navigated().map_err(|e| e.to_string())?;

    // Wait for fonts
    let font_wait_js = r#"
        async function waitForFonts(maxWaitMs = 5000) {
            const start = Date.now();
            while (Date.now() - start < maxWaitMs) {
                if (document.fonts.status === 'loaded') return true;
                await new Promise(r => setTimeout(r, 100));
            }
            return document.fonts.status === 'loaded';
        }
        waitForFonts();
    "#;
    let _ = tab.evaluate(font_wait_js, true);
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Use clip to capture exactly the viewport bounds
    let clip = Viewport {
        x: 0.0,
        y: 0.0,
        width: 800.0,
        height: 1000.0,
        scale: 1.0,
    };

    let screenshot_png = tab
        .capture_screenshot(CaptureScreenshotFormatOption::Png, None, Some(clip), true)
        .map_err(|e| e.to_string())?;

    fs::write(output_path, screenshot_png).map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn export_slides(
    html_path: &str,
    output_dir: &str,
    total_slides: usize,
    width: u32,
    height: u32,
) -> Result<Vec<String>, String> {
    let out = Path::new(output_dir);
    fs::create_dir_all(out).map_err(|e| e.to_string())?;

    let abs_html_path = fs::canonicalize(html_path)
        .map_err(|e| format!("Could not canonicalize HTML path: {}", e))?;
    let file_url = format!("file://{}", abs_html_path.to_string_lossy());

    let ops = optimized_launch_options()?;

    let browser = Browser::new(ops).map_err(|e| format!("{} ({})", CHROME_INSTALL_HINT, e))?;
    let tab = browser.new_tab().map_err(|e| format!("Failed to open browser tab: {}", e))?;

    use headless_chrome::types::Bounds;
    tab.set_bounds(Bounds::Normal {
        left: None,
        top: None,
        width: Some(width as f64),
        height: Some(height as f64),
    })
    .map_err(|e| e.to_string())?;

    tab.navigate_to(&file_url).map_err(|e| e.to_string())?;
    tab.wait_until_navigated().map_err(|e| e.to_string())?;

    // Wait for fonts
    let font_wait_js = r#"
        async function waitForFonts(maxWaitMs = 5000) {
            const start = Date.now();
            while (Date.now() - start < maxWaitMs) {
                if (document.fonts.status === 'loaded') return true;
                await new Promise(r => setTimeout(r, 100));
            }
            return document.fonts.status === 'loaded';
        }
        waitForFonts();
    "#;
    let _ = tab.evaluate(font_wait_js, true);
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let hide_frame_js = format!(
        r#"
        document.querySelectorAll('.ig-header,.ig-dots,.ig-actions,.ig-caption')
            .forEach(el => el.style.display = 'none');
        const frame = document.querySelector('.ig-frame');
        if (frame) {{
            frame.style.cssText = 'width:{}px;height:{}px;max-width:none;border-radius:0;box-shadow:none;overflow:hidden;margin:0;';
        }}
        const viewport = document.querySelector('.carousel-viewport');
        if (viewport) {{
            viewport.style.cssText = 'width:{}px;height:{}px;aspect-ratio:unset;overflow:hidden;cursor:default;';
        }}
        document.body.style.cssText = 'padding:0;margin:0;display:block;overflow:hidden;background:#fff;';
        "#,
        width, height, width, height
    );
    let _ = tab.evaluate(&hide_frame_js, false);
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

    let mut paths = Vec::new();
    for i in 0..total_slides {
        let swipe_js = format!(
            r#"
            const track = document.querySelector('.carousel-track');
            track.style.transition = 'none';
            track.style.transform = 'translateX(-{}px)';
            "#,
            i * width as usize
        );
        let _ = tab.evaluate(&swipe_js, false);

        // Reflow wait
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        // Use clip to capture exactly the requested dimensions
        let clip = Viewport {
            x: 0.0,
            y: 0.0,
            width: width as f64,
            height: height as f64,
            scale: 1.0,
        };

        let screenshot_png = tab
            .capture_screenshot(CaptureScreenshotFormatOption::Png, None, Some(clip), true)
            .map_err(|e| e.to_string())?;

        let slide_name = format!("slide_{}.png", i + 1);
        let slide_path = out.join(&slide_name);
        fs::write(&slide_path, screenshot_png).map_err(|e| e.to_string())?;
        paths.push(slide_path.to_string_lossy().to_string());
    }

    Ok(paths)
}