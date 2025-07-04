use anyhow::{Result, anyhow};
use cli_log::info;
use input::handle_input;
use message::Message;
use state::{State, Timeline};
use tokio::sync::mpsc::{Receiver, Sender, channel};

use crate::{pleroma::tweet::Tweet, renderer::terminal::Terminal};

pub mod backend;
pub mod input;
pub mod message;
mod state;
mod timeline;

#[derive(Default)]
struct Timelines {
    home: Vec<Tweet>,
    // Just your instance
    local: Vec<Tweet>,
    // Everywhere
    public: Vec<Tweet>,
}

pub struct App {
    timelines: Timelines,
    state: State,
    backend_chan: Option<Sender<Message>>,
    pub recv_end: Receiver<Message>,
    pub send_end: Sender<Message>,
    terminal: Terminal,
    instance: String,
    should_render: bool,
}

impl App {
    pub async fn new(instance: &str) -> Result<Self> {
        let (send_end, recv_end) = channel(10);
        Ok(App {
            timelines: Timelines::default(),
            state: State::Timeline(Timeline::Home, 0),
            backend_chan: None,
            recv_end,
            send_end,
            terminal: Terminal::new()?,
            instance: instance.to_string(),
            should_render: true,
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        self.backend_chan
            .as_ref()
            .unwrap()
            .send(Message::GetHomeTimeline(None))
            .await?;

        while !self.recv_end.is_closed() {
            if let Some(m) = self.recv_end.recv().await {
                let should_render = match m {
                    Message::Tick => false,
                    _ => true,
                };
                match m {
                    Message::GetHomeTimelineResponse(res) => match res {
                        Ok(data) => self.timelines.home.extend(data),
                        // TODO Display error on the frontend
                        Err(_) => todo!(),
                    },
                    Message::GetPublicTimelineResponse(res) => match res {
                        Ok(data) => self.timelines.public.extend(data),
                        Err(_) => todo!(),
                    },
                    Message::GetLocalTimelineResponse(res) => match res {
                        Ok(data) => self.timelines.local.extend(data),
                        Err(_) => todo!(),
                    },
                    Message::Tick if self.should_render => {
                        if self.should_render {
                            self.terminal.draw(|frame| todo!())?;
                        }
                    }
                    Message::Input(e) => {
                        info!("Receive input");
                        handle_input(self, e).await?;
                    }
                    _ => (),
                }
                self.should_render = should_render;
            } else {
                return Err(anyhow!("Channel was closed"));
            }
        }

        ratatui::restore();
        Ok(())
    }

    pub async fn register_backend(&mut self, backend: Sender<Message>) {
        self.backend_chan = Some(backend);
    }
}
