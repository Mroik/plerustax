use std::{
    io::{Write, stdin, stdout},
    time::Duration,
};

use anyhow::Result;
use app::{App, message::Message};
use cli_log::init_cli_log;
use pleroma::api::Api;
use tokio::{sync::mpsc::UnboundedSender, task::JoinSet, time::sleep};

mod app;
mod pleroma;

const TICK_RATE: u64 = 1000 / 25;
const INSTANCE: &str = "https://cawfee.club";

#[tokio::main]
async fn main() -> Result<()> {
    init_cli_log!();
    let mut buf = String::new();
    let mut api = Api::new(INSTANCE).await.unwrap();

    print!("Username: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut buf).unwrap();
    let username = buf.trim().to_string();
    buf.clear();

    print!("Password: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut buf).unwrap();
    let password = buf.trim().to_string();

    api.login(&username, &password).await?;

    let mut backend = api.backend().await;
    let mut app = App::new(INSTANCE).await;
    backend.register_app(app.send_end.clone()).await;
    app.register_backend(backend.send_end.clone()).await;

    drop(buf);
    drop(username);
    drop(password);

    let mut threads = JoinSet::new();

    let tick_app = app.send_end.clone();
    threads.spawn(async move { backend.start().await });
    threads.spawn(async move { app.start().await });
    threads.spawn(async move { start_tick_generator(tick_app).await });

    threads.join_all().await;

    Ok(())
}

async fn start_tick_generator(app: UnboundedSender<Message>) -> Result<()> {
    while !app.is_closed() {
        app.send(Message::Tick)?;
        sleep(Duration::from_millis(TICK_RATE)).await;
    }
    Ok(())
}
