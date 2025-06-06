use std::io::Stdout;

use anyhow::{Result, anyhow};
use cli_log::info;
use input::{handle_input, input_generator};
use message::Message;
use ratatui::{
    Terminal,
    prelude::CrosstermBackend,
    widgets::{Block, Borders},
};
use state::{State, Timeline};
use timeline::TimelineWidget;
use tokio::{
    select,
    sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
};

use crate::pleroma::tweet::Tweet;

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
    backend_chan: Option<UnboundedSender<Message>>,
    pub recv_end: UnboundedReceiver<Message>,
    pub send_end: UnboundedSender<Message>,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    instance: String,
}

impl App {
    pub async fn new(instance: &str) -> Self {
        let (send_end, recv_end) = unbounded_channel();
        App {
            timelines: Timelines::default(),
            state: State::Timeline(Timeline::Home, 0),
            backend_chan: None,
            recv_end,
            send_end,
            terminal: ratatui::init(),
            instance: instance.to_string(),
        }
    }

    async fn handle_events(&mut self) -> Result<()> {
        if let Some(m) = self.recv_end.recv().await {
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
                Message::Tick => {
                    // TODO Error handling
                    self.terminal
                        .draw(|frame| match &self.state {
                            State::Timeline(Timeline::Home, i) => {
                                let timeline =
                                    TimelineWidget::new(*i, self.timelines.home.iter().collect());
                                let bl = Block::new()
                                    .borders(Borders::all())
                                    .title(self.instance.clone());
                                let timeline_area = bl.inner(frame.area());

                                frame.render_widget(bl, frame.area());
                                frame.render_widget(timeline, timeline_area);
                            }
                            _ => (),
                        })
                        .unwrap();
                }
                Message::Input(e) => {
                    info!("Receive input");
                    handle_input(self, e).await?;
                }
                _ => (),
            }
        } else {
            return Err(anyhow!("Channel was closed"));
        }
        Ok(())
    }

    pub async fn start(&mut self) -> Result<()> {
        self.backend_chan
            .as_ref()
            .unwrap()
            .send(Message::GetHomeTimeline(None))
            .unwrap();

        while !self.recv_end.is_closed() {
            let input_gen_sender = self.send_end.clone();
            select! {
                _ = self.handle_events() => (),
                _ = input_generator(input_gen_sender) => (),
            }
        }
        ratatui::restore();
        Ok(())
    }

    pub async fn register_backend(&mut self, backend: UnboundedSender<Message>) {
        self.backend_chan = Some(backend);
    }
}
