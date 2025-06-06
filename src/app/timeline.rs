use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Widget,
};

use crate::pleroma::tweet::Tweet;

struct TimelineTweetWidget<'a> {
    tweet: &'a Tweet,
}

impl<'a> From<&'a Tweet> for TimelineTweetWidget<'a> {
    fn from(value: &'a Tweet) -> Self {
        TimelineTweetWidget { tweet: value }
    }
}

impl Widget for TimelineTweetWidget<'_> {
    /// Expecting to always be 3 lines
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        buf.set_string(
            area.left(),
            area.top(),
            if self.tweet.account.acct.len() as u16 > area.width {
                self.tweet
                    .account
                    .acct
                    .chars()
                    .take(area.width as usize - 3)
                    .collect::<String>()
            } else {
                self.tweet.account.acct.clone()
            },
            Style::default(),
        );

        buf.set_string(
            area.left(),
            area.top() + 1,
            self.tweet
                .content
                .chars()
                .take(area.width as usize)
                .collect::<String>(),
            Style::default(),
        );

        let spacing: String = (0..(area.width - 6) / 5).map(|_| ' ').collect();
        let buttons = [
            Span::default().content(&spacing),
            Span::default().content(format!("\u{21b5}{}", self.tweet.replies_count)),
            Span::default().content(&spacing),
            Span::default()
                .content(format!("\u{21ba}{}", self.tweet.reblogs_count))
                .style(Style::default().fg(if self.tweet.reblogged {
                    Color::Green
                } else {
                    Color::Reset
                })),
            Span::default().content(&spacing),
            if self.tweet.favourited {
                Span::default()
                    .content(format!("\u{2605}{}", self.tweet.favourites_count))
                    .style(Style::default().fg(Color::Yellow))
            } else {
                Span::default()
                    .content(format!("\u{2606}{}", self.tweet.favourites_count))
                    .style(Style::default().fg(Color::Reset))
            },
        ];

        buf.set_line(
            area.left(),
            area.top() + 2,
            &Line::default().spans(buttons.iter().cloned()),
            area.width,
        );
    }
}

struct TimelineWidget<'a> {
    tweets: Vec<&'a Tweet>,
    i: usize,
}

impl<'a> TimelineWidget<'a> {
    async fn new(i: usize, tweets: Vec<&'a Tweet>) -> Self {
        TimelineWidget { tweets, i }
    }
}

impl Widget for TimelineWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        self.tweets
            .iter()
            .skip(self.i)
            .take(area.height as usize / 4)
            .map(|&tweet| TimelineTweetWidget::from(tweet))
            .enumerate()
            .for_each(|(i, tweet)| {
                let tweet_area = Rect::new(area.x, area.y + 4 * i as u16, area.width, 3);
                tweet.render(tweet_area, buf);
                buf.set_string(
                    area.x,
                    area.y + 4 * i as u16 + 3,
                    (0..area.width).map(|_| '-').collect::<String>(),
                    Style::default(),
                );
            });
    }
}
