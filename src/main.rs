use std::io::{Write, stdin, stdout};

use anyhow::Result;
use pleroma::api::Api;

mod pleroma;

#[tokio::main]
async fn main() -> Result<()> {
    let mut buf = String::new();
    let mut backend = Api::new("https://cawfee.club").await.unwrap();

    print!("Username: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut buf).unwrap();
    let username = buf.trim().to_string();
    buf.clear();

    print!("Password: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut buf).unwrap();
    let password = buf.trim().to_string();

    backend.login(&username, &password).await?;

    let tweets = backend.home_timeline(None).await?;
    println!("{:?}", tweets);

    Ok(())
}
