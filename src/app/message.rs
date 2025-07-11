use anyhow::Result;
use ratatui::crossterm::event::Event;

use crate::pleroma::tweet::Tweet;

pub enum Message {
    GetHomeTimeline(Option<String>),
    GetHomeTimelineResponse(Result<Vec<Tweet>>),
    GetPublicTimeline(Option<String>),
    GetPublicTimelineResponse(Result<Vec<Tweet>>),
    GetLocalTimeline(Option<String>),
    GetLocalTimelineResponse(Result<Vec<Tweet>>),
    Tick,
    Input(Event),
}
