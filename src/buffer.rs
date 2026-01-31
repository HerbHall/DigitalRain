//! Screen buffer for composing frames before flushing to the terminal.
//!
//! Instead of writing each character individually (which would be slow),
//! we compose the entire frame in memory, then flush it all at once using
//! crossterm's queue! macro for batched output.

use std::io::{self, Write};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
};

/// A single cell on the screen: one character with foreground and background colors.
#[derive(Clone, Copy)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: Color::Reset,
            bg: Color::Reset,
        }
    }
}

/// A 2D grid of cells representing one frame of the display.
pub struct ScreenBuffer {
    width: u16,
    height: u16,
    /// Current frame's cells, stored in row-major order: index = y * width + x
    cells: Vec<Cell>,
    /// Previous frame's cells, used for dirty-checking (only redraw changed cells)
    prev_cells: Vec<Cell>,
    /// Whether this is the first frame (forces a full redraw)
    first_frame: bool,
}

impl ScreenBuffer {
    /// Create a new buffer with the given dimensions.
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width as usize) * (height as usize);
        Self {
            width,
            height,
            cells: vec![Cell::default(); size],
            prev_cells: vec![Cell::default(); size],
            first_frame: true,
        }
    }

    /// Resize the buffer. Clears all cells.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        let size = (width as usize) * (height as usize);
        self.cells = vec![Cell::default(); size];
        self.prev_cells = vec![Cell::default(); size];
        self.first_frame = true;
    }

    /// Clear all cells to spaces with default colors.
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = Cell::default();
        }
    }

    /// Set a single cell. Does nothing if coordinates are out of bounds.
    pub fn set_cell(&mut self, x: u16, y: u16, ch: char, fg: Color, bg: Color) {
        if x < self.width && y < self.height {
            let idx = (y as usize) * (self.width as usize) + (x as usize);
            self.cells[idx] = Cell { ch, fg, bg };
        }
    }

    /// Get a cell at the given coordinates. Returns None if out of bounds.
    pub fn get_cell(&self, x: u16, y: u16) -> Option<&Cell> {
        if x < self.width && y < self.height {
            let idx = (y as usize) * (self.width as usize) + (x as usize);
            Some(&self.cells[idx])
        } else {
            None
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    /// Flush the buffer to the terminal, only writing cells that changed.
    ///
    /// This is the key performance optimization: by comparing against the
    /// previous frame, we only send escape sequences for cells that actually
    /// changed, dramatically reducing I/O.
    pub fn flush(&mut self) -> io::Result<()> {
        let mut stdout = io::stdout();

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y as usize) * (self.width as usize) + (x as usize);
                let cell = self.cells[idx];
                let prev = self.prev_cells[idx];

                // Skip cells that haven't changed (unless first frame)
                if !self.first_frame
                    && cell.ch == prev.ch
                    && color_eq(cell.fg, prev.fg)
                    && color_eq(cell.bg, prev.bg)
                {
                    continue;
                }

                // Queue the draw commands (batched, not flushed yet)
                queue!(
                    stdout,
                    MoveTo(x, y),
                    SetForegroundColor(cell.fg),
                    SetBackgroundColor(cell.bg),
                    Print(cell.ch)
                )?;
            }
        }

        // Send everything to the terminal in one write
        stdout.flush()?;

        // Swap: current becomes previous for next frame's comparison
        std::mem::swap(&mut self.cells, &mut self.prev_cells);
        // Clear current for the next frame to compose into
        for cell in &mut self.cells {
            *cell = Cell::default();
        }

        self.first_frame = false;
        Ok(())
    }
}

/// Compare two crossterm Colors for equality.
/// crossterm::style::Color doesn't implement PartialEq for all variants,
/// so we compare the debug representations as a simple workaround.
fn color_eq(a: Color, b: Color) -> bool {
    // For the color variants we use (Rgb and Reset), direct matching works
    match (a, b) {
        (Color::Reset, Color::Reset) => true,
        (
            Color::Rgb {
                r: r1,
                g: g1,
                b: b1,
            },
            Color::Rgb {
                r: r2,
                g: g2,
                b: b2,
            },
        ) => r1 == r2 && g1 == g2 && b1 == b2,
        (Color::Black, Color::Black)
        | (Color::Red, Color::Red)
        | (Color::Green, Color::Green)
        | (Color::Yellow, Color::Yellow)
        | (Color::Blue, Color::Blue)
        | (Color::Magenta, Color::Magenta)
        | (Color::Cyan, Color::Cyan)
        | (Color::White, Color::White)
        | (Color::DarkRed, Color::DarkRed)
        | (Color::DarkGreen, Color::DarkGreen)
        | (Color::DarkYellow, Color::DarkYellow)
        | (Color::DarkBlue, Color::DarkBlue)
        | (Color::DarkMagenta, Color::DarkMagenta)
        | (Color::DarkCyan, Color::DarkCyan)
        | (Color::Grey, Color::Grey)
        | (Color::DarkGrey, Color::DarkGrey) => true,
        (Color::AnsiValue(a), Color::AnsiValue(b)) => a == b,
        _ => false,
    }
}
