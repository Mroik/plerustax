use crossterm::style::Color;

pub enum PreparedPixel {
    Pixel(Pixel),
    Spaces(u16),
    NewLine(u16),
}

/// A character on the terminal
pub struct Pixel {
    pub char: char,
    pub x: u16,
    pub y: u16,
    pub fg: Color,
    pub bg: Color,
}

impl Pixel {
    pub fn new(char: char) -> Self {
        Pixel {
            char,
            x: 0,
            y: 0,
            fg: Color::Reset,
            bg: Color::Reset,
        }
    }

    pub fn fg(mut self, fg: Color) -> Self {
        self.fg = fg;
        self
    }

    pub fn bg(mut self, bg: Color) -> Self {
        self.bg = bg;
        self
    }

    pub fn set_coords(mut self, x: u16, y: u16) -> Self {
        self.x = x;
        self.y = y;
        self
    }
}

/// The portion of screen to draw on
pub struct Area {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

pub trait Drawable {
    /// The assurance of drawing within the bounds is
    /// responsibility of the user. Whoever calls render
    /// doesn't make guarantees.
    ///
    /// This might change in the future.
    fn render(&self, frame: Area);
}
