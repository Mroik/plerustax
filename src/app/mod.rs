use message::Message;
use state::State;
use tokio::sync::mpsc::UnboundedSender;

use crate::pleroma::tweet::Tweet;

mod message;
mod state;

struct Timelines {
    home: Vec<Tweet>,
    // Just your instance
    public: Vec<Tweet>,
    // Everywhere
    known_network: Vec<Tweet>,
}

struct App {
    timelines: Timelines,
    state: State,
    backend_chan: UnboundedSender<Message>,
}
