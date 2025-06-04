use message::Message;
use state::State;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

use crate::pleroma::tweet::Tweet;

pub mod backend;
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

pub struct App {
    timelines: Timelines,
    state: Vec<State>,
    backend_chan: Option<UnboundedSender<Message>>,
    recv_end: UnboundedReceiver<Message>,
    pub send_end: UnboundedSender<Message>,
}

impl App {
    pub async fn new() -> Self {
        let (send_end, recv_end) = unbounded_channel();
        App {
            timelines: Timelines::default(),
            state: Vec::new(),
            backend_chan: None,
            recv_end,
            send_end,
        }
    }

    pub async fn start(&mut self) {
        while let Some(m) = self.recv_end.recv().await {
            match m {
                Message::GetHomeTimelineResponse(res) => match res {
                    Ok(data) => self.timelines.home.extend(data),
                    // TODO Display error on the frontend
                    Err(_) => todo!(),
                },
                _ => (),
            }
        }
    }

    pub async fn register_backend(&mut self, backend: UnboundedSender<Message>) {
        self.backend_chan = Some(backend);
    }
}
