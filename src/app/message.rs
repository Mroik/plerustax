use anyhow::Result;

use crate::pleroma::tweet::Tweet;

pub enum Message {
    GetHomeTimeline(Option<String>),
    GetHomeTimelineResponse(Result<Vec<Tweet>>),
}
