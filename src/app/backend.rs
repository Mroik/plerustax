use anyhow::Result;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::pleroma::api::Api;

use super::message::Message;

pub struct Backend {
    api: Api,
    app_chan: Option<UnboundedSender<Message>>,
    recv_end: UnboundedReceiver<Message>,
    send_end: UnboundedSender<Message>,
}

impl Backend {
    pub async fn new(api: Api) -> Self {
        let (send_end, recv_end) = tokio::sync::mpsc::unbounded_channel();
        Backend {
            api,
            app_chan: None,
            recv_end,
            send_end,
        }
    }

    async fn start(&mut self) -> Result<()> {
        while let Some(m) = self.recv_end.recv().await {
            match m {
                Message::GetHomeTimeline(id) => {
                    let res = self.api.home_timeline(id.clone().as_deref()).await;
                    self.app_chan
                        .as_ref()
                        .unwrap()
                        .send(Message::GetHomeTimelineResponse(res))?;
                }
                _ => (),
            }
        }
        Ok(())
    }
}
