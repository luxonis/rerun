use crossbeam_channel::{self, Receiver, Sender};
use ewebsock::{WsEvent, WsMessage};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::ControlFlow;
use std::process::exit;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use super::depthai;

// TODO:(filip) make this try to reconnect until a successful connection
async fn spawn_ws_client(
    recv_tx: crossbeam_channel::Sender<WsMessage>,
    send_rx: crossbeam_channel::Receiver<WsMessage>,
    shutdown: Arc<AtomicBool>,
) {
    let (error_tx, error_rx) = crossbeam_channel::unbounded();
    // Retry connection until successful
    loop {
        let recv_tx = recv_tx.clone();
        let error_tx = error_tx.clone();
        if let Ok(sender) = ewebsock::ws_connect(
            String::from("ws://localhost:9001"),
            Box::new(move |event| {
                match event {
                    WsEvent::Opened => {
                        re_log::info!("Websocket opened");
                        ControlFlow::Continue(())
                    }
                    WsEvent::Message(message) => {
                        // re_log::debug!("Websocket message");
                        recv_tx.send(message);
                        ControlFlow::Continue(())
                    }
                    WsEvent::Error(e) => {
                        // re_log::info!("Websocket Error: {:?}", e);
                        error_tx.send(e);
                        ControlFlow::Break(())
                    }
                    WsEvent::Closed => {
                        // re_log::info!("Websocket Closed");
                        error_tx.send(String::from("Websocket Closed"));
                        ControlFlow::Break(())
                    }
                }
            }),
        )
        .as_mut()
        {
            while error_rx.is_empty() {
                if shutdown.load(std::sync::atomic::Ordering::SeqCst) {
                    re_log::debug!("Shutting down websocket client");
                    exit(0);
                }
                if let Ok(message) = send_rx.recv_timeout(std::time::Duration::from_millis(100)) {
                    sender.send(message);
                }
            }
            for error in error_rx.try_iter() {
                re_log::debug!("Websocket error: {:?}", error);
            }
        } else {
            re_log::error!("Coudln't create websocket");
        }
        if shutdown.load(std::sync::atomic::Ordering::SeqCst) {
            re_log::debug!("Shutting down websocket client");
            exit(0);
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

#[derive(Serialize, Deserialize, fmt::Debug)]
pub enum WsMessageData {
    Subscriptions(Vec<depthai::ChannelId>),
    Devices(Vec<depthai::DeviceId>),
    Device(depthai::Device),
    Pipeline(depthai::DeviceConfig),
    Error(String),
}

#[derive(Deserialize, Serialize, fmt::Debug)]
pub enum WsMessageType {
    Subscriptions,
    Devices,
    Device,
    Pipeline,
    Error,
}

impl Default for WsMessageType {
    fn default() -> Self {
        Self::Error
    }
}

#[derive(Serialize, fmt::Debug)]
pub struct BackWsMessage {
    #[serde(rename = "type")]
    pub kind: WsMessageType,
    // #[serde(deserialize_with = "deserialize_ws_message_data")]
    pub data: WsMessageData,
}

impl<'de> Deserialize<'de> for BackWsMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        pub struct Message {
            #[serde(rename = "type")]
            pub kind: WsMessageType,
            pub data: serde_json::Value,
        }

        let message = Message::deserialize(deserializer)?;
        let data = match message.kind {
            WsMessageType::Subscriptions => WsMessageData::Subscriptions(
                serde_json::from_value(message.data).unwrap_or_default(),
            ),
            WsMessageType::Devices => {
                WsMessageData::Devices(serde_json::from_value(message.data).unwrap_or_default())
            }
            WsMessageType::Device => {
                WsMessageData::Device(serde_json::from_value(message.data).unwrap_or_default())
            }
            WsMessageType::Pipeline => {
                WsMessageData::Pipeline(serde_json::from_value(message.data).unwrap())
                // TODO(filip) change to unwrap_or_default when pipeline config api is more stable
            }
            WsMessageType::Error => {
                WsMessageData::Error(serde_json::from_value(message.data).unwrap_or_default())
            }
        };
        Ok(Self {
            kind: message.kind,
            data,
        })
    }
}

impl Default for BackWsMessage {
    fn default() -> Self {
        Self {
            kind: WsMessageType::Error.into(),
            data: WsMessageData::Error(String::from("Invalid message")),
        }
    }
}

pub struct WebSocket {
    receiver: crossbeam_channel::Receiver<WsMessage>,
    sender: crossbeam_channel::Sender<WsMessage>,
    shutdown: Arc<AtomicBool>,
    task: tokio::task::JoinHandle<()>,
}

impl Default for WebSocket {
    fn default() -> Self {
        Self::new()
    }
}

impl WebSocket {
    pub fn new() -> Self {
        let (recv_tx, recv_rx) = crossbeam_channel::unbounded();
        let (send_tx, send_rx) = crossbeam_channel::unbounded();
        let shutdown = Arc::new(AtomicBool::new(false));
        let shutdown_clone = shutdown.clone();
        let task = tokio::spawn(spawn_ws_client(recv_tx, send_rx, shutdown_clone));
        Self {
            receiver: recv_rx,
            sender: send_tx,
            shutdown,
            task,
        }
    }

    pub fn shutdown(&mut self) {
        self.shutdown
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn receive(&self) -> Option<BackWsMessage> {
        if let Ok(message) = self.receiver.try_recv() {
            match message {
                WsMessage::Text(text) => {
                    re_log::debug!("Received: {:?}", text);
                    match serde_json::from_str::<BackWsMessage>(&text.as_str()) {
                        Ok(back_message) => {
                            return Some(back_message);
                        }
                        Err(error) => {
                            re_log::error!("Error: {:?}", error);
                            return None;
                        }
                    }
                }
                _ => return None,
            }
        }
        None
    }
    pub fn send(&self, message: String) {
        self.sender.send(WsMessage::Text(message));
    }
}
