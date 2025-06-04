use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use super::message::Message;

struct Backend {
    app_chan: Option<UnboundedSender<Message>>,
    recv_end: UnboundedReceiver<Message>,
    send_end: UnboundedSender<Message>,
}

impl Backend {
    async fn new() -> Self {
        let (send_end, recv_end) = tokio::sync::mpsc::unbounded_channel();
        Backend {
            app_chan: None,
            recv_end,
            send_end,
        }
    }

    async fn start(&mut self) {
        while let Some(m) = self.recv_end.recv().await {
            match m {
                _ => (),
            }
        }
    }
}
