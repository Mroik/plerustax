use std::time::Duration;

use anyhow::Result;
use cli_log::info;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyModifiers, poll};
use tokio::sync::mpsc::UnboundedSender;

use crate::TICK_RATE;

use super::{
    App,
    message::Message,
    state::{State, Timeline},
};

pub async fn input_generator(app: UnboundedSender<Message>) -> Result<()> {
    if poll(Duration::from_millis(TICK_RATE))? {
        let event = event::read()?;
        info!("Sending event");
        info!("{:?}", event);
        app.send(Message::Input(event))?;
    }
    Ok(())
}

pub async fn handle_input(app: &mut App, event: Event) -> Result<()> {
    info!("Handling input");
    match event {
        Event::Key(e) => match (e.code, e.modifiers) {
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                info!("CTRL-C");
                app.recv_end.close();
                while let Some(_) = app.recv_end.recv().await {}
                return Ok(());
            }
            (_, _) => (),
        },
        _ => (),
    }

    info!("Before match state");
    match app.state.clone() {
        State::Timeline(_, _) => handle_timeline(app, event),
    }
}

/// TODO: Fetch new tweets on list edge
fn handle_timeline(app: &mut App, event: Event) -> Result<()> {
    info!("In timeline input handling");
    match event {
        Event::Key(key_event) => match key_event.code {
            KeyCode::Down => match &mut app.state {
                State::Timeline(t, i) => {
                    let t_len = match t {
                        Timeline::Home => app.timelines.home.len(),
                        Timeline::Local => app.timelines.local.len(),
                        Timeline::Public => app.timelines.public.len(),
                    };
                    if *i < t_len - 1 {
                        *i = *i + 1;
                    }
                }
                _ => unreachable!(),
            },
            KeyCode::Up => match &mut app.state {
                State::Timeline(t, i) => {
                    if *i > 0 {
                        *i = *i - 1;
                    }
                }
                _ => unreachable!(),
            },
            _ => (),
        },
        _ => (),
    }
    Ok(())
}
