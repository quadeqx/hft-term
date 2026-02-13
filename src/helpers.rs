//! src/helpers.rs

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub fn grapheme_to_byte_idx(s: &str, g_idx: usize) -> usize {
    s.grapheme_indices(true)
        .nth(g_idx)
        .map(|(i, _)| i)
        .unwrap_or(s.len())
}

pub fn grapheme_display_width(s: &str) -> u16 {
    UnicodeWidthStr::width(s) as u16
}

pub fn cursor_position_soft_wrapped(
    buffer: &str,
    cursor_grapheme: usize,
    usable_width: u16,
) -> (u16, u16) {
    let mut row: u16 = 0;
    let mut col: u16 = 0;

    for (i, g) in buffer.graphemes(true).enumerate() {
        if i == cursor_grapheme {
            break;
        }

        let w = grapheme_display_width(g).max(1);

        if col + w == usable_width - 1 {
            row += 1;
            col = 0;
        }

        col += w;
    }

    (row, col)
}
