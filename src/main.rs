use std::io::{Write, stdin, stdout};

use anyhow::Result;
use app::App;
use pleroma::api::Api;
use tokio::spawn;

mod app;
mod pleroma;

#[tokio::main]
async fn main() -> Result<()> {
    let mut buf = String::new();
    let mut api = Api::new("https://cawfee.club").await.unwrap();

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
    let mut app = App::new().await;
    backend.register_app(app.send_end.clone()).await;
    app.register_backend(backend.send_end.clone()).await;

    drop(buf);
    drop(username);
    drop(password);

    spawn(async move { backend.start().await });
    spawn(async move { app.start().await });

    Ok(())
}
