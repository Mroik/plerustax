use anyhow::Result;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::pleroma::api::Api;

use super::message::Message;

pub struct Backend {
    api: Api,
    app_chan: Option<UnboundedSender<Message>>,
    recv_end: UnboundedReceiver<Message>,
    pub send_end: UnboundedSender<Message>,
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

    pub async fn start(&mut self) -> Result<()> {
        while !self.app_chan.as_ref().unwrap().is_closed() {
            let message = self.recv_end.recv().await;
            if message.is_none() {
                break;
            }
            match message.unwrap() {
                Message::GetHomeTimeline(id) => {
                    let res = self.api.home_timeline(id.clone().as_deref()).await;
                    self.app_chan
                        .as_ref()
                        .unwrap()
                        .send(Message::GetHomeTimelineResponse(res))?;
                }
                Message::GetPublicTimeline(id) => {
                    let res = self.api.public_timeline(id.clone().as_deref()).await;
                    self.app_chan
                        .as_ref()
                        .unwrap()
                        .send(Message::GetPublicTimelineResponse(res))?;
                }
                Message::GetLocalTimeline(id) => {
                    let res = self.api.local_timeline(id.clone().as_deref()).await;
                    self.app_chan
                        .as_ref()
                        .unwrap()
                        .send(Message::GetLocalTimelineResponse(res))?;
                }
                _ => (),
            }
        }
        Ok(())
    }

    pub async fn register_app(&mut self, app: UnboundedSender<Message>) {
        self.app_chan = Some(app);
    }
}
