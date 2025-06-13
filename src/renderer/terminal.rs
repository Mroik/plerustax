use std::{
    cmp::Ordering,
    io::{Stdout, stdout},
};

use anyhow::Result;
use crossterm::{
    ExecutableCommand,
    style::{Color, Print, Stylize},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};

use super::utils::Pixel;

pub struct Frame<'a> {
    terminal: &'a mut Terminal,
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

    fn frame(&mut self) -> Frame {
        Frame { terminal: self }
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
        let mut f = self.frame();
        callback(&mut f);
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
