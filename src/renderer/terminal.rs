use std::{
    cmp::Ordering,
    io::{Stdout, stdout},
};

use anyhow::Result;
use crossterm::{
    ExecutableCommand,
    cursor::{MoveTo, position},
    style::{Color, Print, Stylize},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::renderer::utils::Drawable;

use super::utils::Pixel;

pub struct Frame<'a> {
    terminal: &'a mut Terminal,
    buffer: Vec<Pixel>,
    top: u16,
    left: u16,
    bottom: u16,
    right: u16,
}

impl Frame<'_> {
    fn write(&mut self, p: Pixel) {
        self.buffer.push(p);
    }

    // TODO: Check bounds
    fn write_str(&mut self, s: &str, x: u16, y: u16, fg: Color, bg: Color) {
        self.buffer
            .extend(s.chars().enumerate().map(|(i, c)| Pixel {
                char: c,
                x: x + i as u16,
                y,
                fg,
                bg,
            }));
    }

    // TODO: Generate new frame to use for widget rendering.
    // This is done to enforce bounds limits.
    fn render_widget(&mut self, widget: impl Drawable) {
        todo!()
    }
}

/// Rapresents the terminal. On instancing it sets the terminal
/// to alternate screen and enables raw mode. On drop it disables
/// and reverts to the original screen.
pub struct Terminal {
    frame_pixels: Vec<Pixel>,
    output: Stdout,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        let mut output = stdout();
        output.execute(EnterAlternateScreen)?;
        crossterm::terminal::enable_raw_mode()?;
        Ok(Terminal {
            frame_pixels: Vec::new(),
            output,
        })
    }

    fn frame(&mut self) -> Result<Frame> {
        self.output.execute(MoveTo(9000, 9000))?;
        let (right, bottom) = position().unwrap();
        Ok(Frame {
            terminal: self,
            buffer: Vec::new(),
            top: 0,
            left: 0,
            right,
            bottom,
        })
    }

    /// Generate the lines to print for the next frame
    fn generate_frame_pixels(&mut self) -> String {
        self.frame_pixels.sort_by(|a, b| match a.y.cmp(&b.y) {
            Ordering::Equal => a.x.cmp(&b.x),
            res => res,
        });

        self.frame_pixels
            .iter()
            .take(self.frame_pixels.len() - 1)
            .zip(self.frame_pixels.iter().skip(1))
            .enumerate()
            .filter(|(_, (a, b))| a.y < b.y)
            .map(|(i, _)| i + 1)
            .enumerate()
            .collect::<Vec<(usize, usize)>>()
            .iter()
            .for_each(|(i, s)| {
                self.frame_pixels.insert(
                    s + i,
                    Pixel {
                        char: '\n',
                        x: 0,
                        y: 0,
                        fg: Color::Reset,
                        bg: Color::Reset,
                    },
                );
            });

        let mut a: Vec<Vec<Pixel>> = self
            .frame_pixels
            .split(|p| p.char == '\n')
            .map(|a| a.iter().cloned().collect::<Vec<Pixel>>())
            .collect();

        self.frame_pixels.clear();

        a.iter_mut().for_each(|line| {
            let mut i = 0;
            while i < line.len() - 1 {
                if i == 0 && line[i].x > 0 {
                    line.insert(
                        i,
                        Pixel {
                            char: ' ',
                            x: 0,
                            y: line[i].y,
                            fg: Color::Reset,
                            bg: Color::Reset,
                        },
                    );
                } else if line[i].x < line[i + 1].x - 1 {
                    line.insert(
                        i + 1,
                        Pixel {
                            char: ' ',
                            x: line[i].x + 1,
                            y: line[i].y,
                            fg: Color::Reset,
                            bg: Color::Reset,
                        },
                    );
                }
                i += 1;
            }
        });

        a.iter()
            .map(|line| {
                line.iter()
                    .map(|p| {
                        let v = p.char.stylize();
                        v.with(p.fg).on(p.bg).to_string()
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Method used to draw the widget on screen
    pub fn draw<T>(&mut self, callback: T) -> Result<()>
    where
        T: FnOnce(&mut Frame),
    {
        let mut f = self.frame()?;
        callback(&mut f);
        let buf = f.buffer;
        self.frame_pixels.extend(buf);
        let ui = self.generate_frame_pixels();
        self.output.execute(Print(ui))?;
        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        // This looks horrible but I don't think there's a way to
        // deal with failing drops in a clean way.
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = self.output.execute(LeaveAlternateScreen);
    }
}
