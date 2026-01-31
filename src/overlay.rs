//! Overlay rendering: keybindings help, status messages, etc.
//!
//! Overlays are drawn on top of the effect buffer after the effect renders.
//! They use a dark background to remain readable over the rain.

use crossterm::style::Color;

use crate::buffer::ScreenBuffer;

/// The dark background color for overlay text boxes.
const OVERLAY_BG: Color = Color::Rgb {
    r: 10,
    g: 10,
    b: 10,
};
/// The main text color for overlay content.
const OVERLAY_FG: Color = Color::Rgb {
    r: 180,
    g: 180,
    b: 180,
};
/// The title/header color for overlay boxes.
const OVERLAY_TITLE: Color = Color::Rgb {
    r: 0,
    g: 200,
    b: 80,
};

/// Render the keybindings help overlay centered on screen.
pub fn render_help(buffer: &mut ScreenBuffer) {
    let lines = [
        "",
        "  KEYBINDINGS",
        "",
        "  Space     Pause / Resume",
        "  +  -      Speed up / down",
        "  [  ]      Density down / up",
        "  n         Next effect",
        "  r         Randomize",
        "  t         Toggle auto-cycle timer",
        "  ?         Toggle this help",
        "  q / Esc   Quit",
        "",
    ];

    let box_width = 38u16;
    let box_height = lines.len() as u16;

    let buf_w = buffer.width();
    let buf_h = buffer.height();

    // Don't render if terminal is too small
    if buf_w < box_width + 4 || buf_h < box_height + 2 {
        return;
    }

    let start_x = (buf_w - box_width) / 2;
    let start_y = (buf_h - box_height) / 2;

    for (row, line) in lines.iter().enumerate() {
        let y = start_y + row as u16;

        // Fill the full box width with background
        for x in start_x..(start_x + box_width) {
            let col = (x - start_x) as usize;
            let ch = line.chars().nth(col).unwrap_or(' ');

            // Title line gets a different color
            let fg = if row == 1 { OVERLAY_TITLE } else { OVERLAY_FG };

            buffer.set_cell(x, y, ch, fg, OVERLAY_BG);
        }
    }
}

/// Render a brief status message at the bottom of the screen.
/// Used to show parameter changes ("Speed: 1.5x") that fade after a moment.
pub fn render_status(buffer: &mut ScreenBuffer, message: &str) {
    let buf_w = buffer.width();
    let buf_h = buffer.height();

    if buf_h < 1 || buf_w < 10 {
        return;
    }

    let y = buf_h - 1;
    let msg_len = message.len().min(buf_w as usize);
    let start_x = (buf_w as usize - msg_len) / 2;

    // One space padding on each side
    let pad_start = if start_x > 0 { start_x - 1 } else { 0 };
    let pad_end = (start_x + msg_len + 1).min(buf_w as usize);

    for x in pad_start..pad_end {
        let col = x as isize - start_x as isize;
        let ch = if col >= 0 && (col as usize) < msg_len {
            message.chars().nth(col as usize).unwrap_or(' ')
        } else {
            ' '
        };
        buffer.set_cell(x as u16, y, ch, OVERLAY_TITLE, OVERLAY_BG);
    }
}
