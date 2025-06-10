use std::io::{Stdout, stdout};

use anyhow::Result;
use crossterm::{
    ExecutableCommand, QueueableCommand,
    style::{Color, PrintStyledContent, Stylize},
    terminal::{
        BeginSynchronizedUpdate, EndSynchronizedUpdate, EnterAlternateScreen, LeaveAlternateScreen,
    },
};

use super::utils::{Pixel, PreparedPixel};

struct Frame<'a> {
    terminal: &'a mut Terminal,
}

/// Rapresents the terminal. On instancing it sets the terminal
/// to alternate screen and enables raw mode. On drop it disables
/// and reverts to the original screen.
struct Terminal {
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

    /// Generate the list of pixels to draw on the next frame.
    /// Multiple spaces are accumulated on the same String.
    fn generate_frame_pixels(&self) -> Vec<PreparedPixel> {
        // TODO
        todo!()
    }

    pub fn draw<T>(&mut self, callback: T)
    where
        T: FnOnce(&mut Frame),
    {
        let mut f = self.frame();
        callback(&mut f);
        let _ = self.output.execute(BeginSynchronizedUpdate);

        // Could move this code to Frame's drop, that way
        // the draw method wouldn't be needed and the error
        // handling would be cleaner.
        self.generate_frame_pixels()
            .iter()
            .for_each(|pix| match pix {
                PreparedPixel::Pixel(pixel) => {
                    let mut cont = pixel.char.stylize();
                    cont = match pixel.fg {
                        Color::Black => cont.black(),
                        Color::Red => cont.red(),
                        Color::Green => cont.green(),
                        Color::Yellow => cont.yellow(),
                        Color::Blue => cont.blue(),
                        Color::Magenta => cont.magenta(),
                        Color::Cyan => cont.cyan(),
                        Color::White => cont.white(),
                        Color::Grey => cont.grey(),
                        _ => unreachable!(),
                    };
                    cont = match pixel.fg {
                        Color::Black => cont.black(),
                        Color::Red => cont.red(),
                        Color::Green => cont.green(),
                        Color::Yellow => cont.yellow(),
                        Color::Blue => cont.blue(),
                        Color::Magenta => cont.magenta(),
                        Color::Cyan => cont.cyan(),
                        Color::White => cont.white(),
                        Color::Grey => cont.grey(),
                        _ => unreachable!(),
                    };
                    let _ = self.output.queue(PrintStyledContent(cont));
                }
                PreparedPixel::Spaces(n) => {
                    let spaces = (0..*n).map(|_| ' ').collect::<String>();
                    let _ = self.output.queue(crossterm::style::Print(spaces));
                }
                PreparedPixel::NewLine(n) => {
                    let lines = (0..*n).map(|_| '\n').collect::<String>();
                    let _ = self.output.queue(crossterm::style::Print(lines));
                }
            });

        let _ = self.output.execute(EndSynchronizedUpdate);
        self.frame_pixels.clear();
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
