use message::Message;
use state::State;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

use crate::pleroma::tweet::Tweet;

mod backend;
mod message;
mod state;

#[derive(Default)]
struct Timelines {
    home: Vec<Tweet>,
    // Just your instance
    public: Vec<Tweet>,
    // Everywhere
    known_network: Vec<Tweet>,
}

struct App {
    timelines: Timelines,
    state: Vec<State>,
    backend_chan: Option<UnboundedSender<Message>>,
    recv_end: UnboundedReceiver<Message>,
    send_end: UnboundedSender<Message>,
}

impl App {
    async fn new() -> Self {
        let (send_end, recv_end) = unbounded_channel();
        App {
            timelines: Timelines::default(),
            state: Vec::new(),
            backend_chan: None,
            recv_end,
            send_end,
        }
    }
}
