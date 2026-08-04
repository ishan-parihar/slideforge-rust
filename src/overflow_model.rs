//! Shared text-overflow model.
//!
//! Single source of truth for estimating whether text fits the 420×525 base
//! composition. Used by BOTH the renderer (`components`/`blocks`, for automatic
//! component scaling) and the validator (`validate`, for the compile-time and
//! post-generation gates) so the two sides can never drift apart — a slide the
//! renderer considers fitted is exactly what the validator's gate accepts.
//!
//! Geometry model (matches `slide_base`/`hero_layout` behavior):
//! - Composition is 420×525.
//! - The chrome is split into REAL bands: a `.slide-header` band (36px, brand +
//!   topic corners) and a `.slide-footer` band (40px, url + hashtags corners +
//!   progress bar). Slide types render ONLY inside the `.slide-body` region
//!   between the bands. Available content height =
//!   `525 − header(36) − footer(40) − slide-content padding`, never exceeding
//!   `SAFE_CONTENT_HEIGHT` (525 − 36 − 40 = 449) for padding-less layouts.
//! - Wrapped line count is estimated from an average glyph advance of `0.55 ×
//!   font-size` (a standard heuristic for mixed-case Latin text).

/// Base composition height (px).
pub const COMPOSITION_HEIGHT: f32 = 525.0;
/// `.slide-header` band height: brand (left) + topic (right) corners.
pub const CHROME_HEADER_HEIGHT: f32 = 36.0;
/// `.slide-footer` band height: url (left) + hashtags (right) + progress bar.
pub const CHROME_FOOTER_HEIGHT: f32 = 40.0;
/// Total banded chrome: header + footer.
pub const HEADER_FOOTER_SPACE: f32 = CHROME_HEADER_HEIGHT + CHROME_FOOTER_HEIGHT; // 76.0
/// Maximum usable content height for padding-less layouts (body region only).
pub const SAFE_CONTENT_HEIGHT: f32 = COMPOSITION_HEIGHT - HEADER_FOOTER_SPACE; // 449.0
/// Average glyph width as a fraction of font-size.
pub const AVG_CHAR_WIDTH_FACTOR: f32 = 0.55;
/// Default text column width when a style does not constrain it (420 − 2×44 sides).
pub const DEFAULT_COLUMN_WIDTH: f32 = 332.0;
/// Blockquote text column width (quote_slide glass card: 332 − 2×32 `--space-4`).
/// Shared by the renderer's quote fit AND the validator's estimate so the two
/// sides cannot drift apart (single calibration point).
pub const QUOTE_COLUMN_WIDTH: f32 = 272.0;
/// Text height budget inside the quote glass card (fits quote text only, before
/// the ~180px fixed chrome). Calibrated against headless-Chromium measurement:
/// 200px and 160px both overflowed; 128px leaves a safe margin.
pub const QUOTE_TEXT_BUDGET: f32 = 128.0;
/// Fixed quote chrome (decorative mark + divider + attribution + glass padding)
/// that the validator adds on top of the text sum.
pub const QUOTE_CHROME_HEIGHT: f32 = 180.0;

/// Estimated number of wrapped lines for `text` at `font_size` within `width`.
pub fn estimate_wrapped_lines(text: &str, font_size: f32, width: f32) -> usize {
    let visible: usize = text.chars().filter(|c| *c != '\n').count();
    let explicit_newlines: usize = text.chars().filter(|c| *c == '\n').count();
    let chars_per_line = (width / (font_size.max(1.0) * AVG_CHAR_WIDTH_FACTOR))
        .floor()
        .max(1.0);
    let wrapped = (visible as f32 / chars_per_line).ceil() as usize;
    wrapped + explicit_newlines
}

/// Estimated rendered text height for `text` at `font_size`/`line_height` in `width`.
///
/// `line_height` may be unitless (≤ 4.0, multiplied by font-size) or an absolute
/// px value (consistent with CSS semantics).
pub fn estimate_text_height(text: &str, font_size: f32, line_height: f32, width: f32) -> f32 {
    let line_height_px = if line_height <= 4.0 {
        line_height * font_size
    } else {
        line_height
    };
    estimate_wrapped_lines(text, font_size, width) as f32 * line_height_px
}

/// Largest font size in `[min, max]` (stepping by `step`) whose estimated wrapped
/// height for `text` in `width` fits within `max_height` at `line_height`.
pub fn fit_font_size(
    text: &str,
    width: f32,
    max_height: f32,
    line_height: f32,
    min_size: f32,
    max_size: f32,
) -> f32 {
    let mut size = max_size;
    while size >= min_size {
        if estimate_text_height(text, size, line_height, width) <= max_height {
            return size;
        }
        size -= 2.0;
    }
    min_size
}

/// Largest font size in `[min, max]` (stepping by `step`) whose estimated wrapped
/// line count for `text` in `width` is at most `max_lines` at `line_height`.
pub fn fit_font_size_to_lines(
    text: &str,
    width: f32,
    max_lines: usize,
    line_height: f32,
    min_size: f32,
    max_size: f32,
) -> f32 {
    let mut size = max_size;
    while size >= min_size {
        if estimate_wrapped_lines(text, size, width) <= max_lines {
            return size;
        }
        size -= 2.0;
    }
    min_size
}

/// Largest font size (stepping by `step`) that keeps EVERY word of `text` on a
/// single line within `width`. Words are never split mid-character — a long
/// single word gets the font scaled down until it fits whole. This is the
/// word-integrity complement to `fit_font_size_to_lines` (which allows
/// mid-word breaks).
pub fn fit_font_size_to_words(
    text: &str,
    width: f32,
    min_size: f32,
    max_size: f32,
    step: f32,
) -> f32 {
    let longest_word_chars = text
        .split_whitespace()
        .map(|w| w.chars().count())
        .max()
        .unwrap_or(1)
        .max(1) as f32;
    let mut size = max_size;
    // Average glyph advance ≈ 0.55 × font-size for 900-weight display faces.
    while size >= min_size {
        if longest_word_chars * size * 0.55 <= width {
            return size;
        }
        size -= step;
    }
    min_size
}

/// Available content height given the slide-content vertical padding, within the
/// `.slide-body` region (composition minus the 36px header and 40px footer
/// chrome bands). Padding-less layouts still reserve the chrome.
pub fn available_content_height(padding_top: f32, padding_bottom: f32) -> f32 {
    (COMPOSITION_HEIGHT - CHROME_HEADER_HEIGHT - CHROME_FOOTER_HEIGHT - padding_top - padding_bottom)
        .max(0.0)
        .min(SAFE_CONTENT_HEIGHT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_text_single_line() {
        assert_eq!(estimate_wrapped_lines("hello", 16.0, 320.0), 1);
    }

    #[test]
    fn long_text_wraps() {
        // 113px in a 272px column ≈ 4.4 chars/line → 40 chars ≈ 10 lines.
        let lines = estimate_wrapped_lines(
            "Design is the silent language of trust.",
            113.0,
            272.0,
        );
        assert!(lines >= 8, "expected heavy wrap, got {lines}");
    }

    #[test]
    fn fit_shrinks_oversized_quote() {
        let quote = "Design is the silent language of trust.";
        // 113px would overflow; the fit must return a size that fits 167px.
        let fitted = fit_font_size(quote, 272.0, 167.0, 1.25, 26.0, 113.0);
        assert!(fitted < 113.0, "expected a shrunken size, got {fitted}");
        let h = estimate_text_height(quote, fitted, 1.25, 272.0);
        assert!(h <= 167.0, "fitted size {fitted} still overflows: {h}");
    }

    #[test]
    fn fit_respects_line_cap() {
        let headline = "A bolder brief, built for the feed";
        let fitted = fit_font_size_to_lines(headline, 320.0, 2, 1.02, 28.0, 113.0);
        assert!(
            estimate_wrapped_lines(headline, fitted, 320.0) <= 2,
            "fitted {fitted} exceeds 2 lines"
        );
    }

    #[test]
    fn available_height_caps_at_safe() {
        assert_eq!(available_content_height(0.0, 0.0), SAFE_CONTENT_HEIGHT);
        assert_eq!(available_content_height(60.0, 60.0), 329.0);
        assert_eq!(available_content_height(80.0, 80.0), 289.0);
        assert_eq!(available_content_height(16.0, 20.0), 413.0);
    }

    #[test]
    fn words_fit_whole_longest_word() {
        // "Beautiful by default" — longest word is "Beautiful" (9 chars).
        // In a 164px column, 9 × size × 0.55 ≤ 164 → size ≤ 33.1.
        let fitted = fit_font_size_to_words("Beautiful by default", 164.0, 16.0, 60.0, 2.0);
        assert!(fitted <= 33.1, "longest word would split at {fitted}");
        assert!(fitted >= 31.0, "too aggressive: {fitted}");
    }

    #[test]
    fn words_fit_single_long_word() {
        // Single word case: must shrink hard so the word stays whole.
        let fitted = fit_font_size_to_words("Extraordinary", 164.0, 10.0, 60.0, 2.0);
        // 13 chars × size × 0.55 ≤ 164 → size ≤ 22.9
        assert!(fitted <= 23.0, "word splits at {fitted}");
    }
}
