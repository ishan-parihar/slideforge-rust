use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;
use headless_chrome::{Browser, LaunchOptions};
use std::fs;
use std::path::Path;

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

    let ops = LaunchOptions::default_builder()
        .headless(true)
        .build()
        .map_err(|e| e.to_string())?;

    let browser = Browser::new(ops).map_err(|e| e.to_string())?;
    let tab = browser.new_tab().map_err(|e| e.to_string())?;

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

        let screenshot_png = tab
            .capture_screenshot(CaptureScreenshotFormatOption::Png, None, None, true)
            .map_err(|e| e.to_string())?;

        let slide_name = format!("slide_{}.png", i + 1);
        let slide_path = out.join(&slide_name);
        fs::write(&slide_path, screenshot_png).map_err(|e| e.to_string())?;
        paths.push(slide_path.to_string_lossy().to_string());
    }

    Ok(paths)
}
