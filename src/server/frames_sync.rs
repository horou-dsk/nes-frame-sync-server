use tokio::sync::mpsc::{Sender, Receiver};
use std::time::Duration;
use chrono::Local;
use crate::server::messages::Message;
use actix::Recipient;
use bytes::{Bytes, BytesMut, BufMut};

const MS_PER_UPDATE: f64 = 100000000.0 / 6.0;

#[derive(Debug)]
pub enum FrameMessage {
    Stop,
    KeyBuffer(Bytes),
    Frame,
}

#[derive(Debug)]
pub struct Frames {
    running: bool,
    tx: Sender<FrameMessage>,
    rx: Option<Receiver<FrameMessage>>
}

impl Frames {
    pub fn new() -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(128);
        Self {
            running: false,
            tx,
            rx: Some(rx),
        }
    }

    pub fn start(&mut self, addrs: Vec<Recipient<Message>>) {
        if self.running {
            return
        }
        self.running = true;
        let mut rx = match self.rx.take() {
            Some(rx) => rx,
            None => {
                let (tx, rx) = tokio::sync::mpsc::channel(128);
                self.tx = tx;
                rx
            }
        };
        tokio::spawn(async move {
            let mut frame_buffer = BytesMut::new();
            'out: loop {
                if let Some(msg) = rx.recv().await {
                    match msg {
                        FrameMessage::Stop => {
                            break
                        },
                        FrameMessage::Frame => {
                            for addr in addrs.iter() {
                                match addr.do_send(Message::Binary(Bytes::from(frame_buffer.clone()))) {
                                    Err(_) => break 'out,
                                    _ => {}
                                }
                            }
                            frame_buffer.clear();
                        }
                        FrameMessage::KeyBuffer(bytes) => {
                            frame_buffer.put(bytes)
                        }
                    }
                }
            }

        });
        let tx = self.tx.clone();
        tokio::spawn(async move {
            let mut next_game_tick = Local::now().timestamp_nanos() as f64;
            loop {
                next_game_tick += MS_PER_UPDATE;
                let sleep_time = next_game_tick - Local::now().timestamp_nanos() as f64;
                match tx.send(FrameMessage::Frame).await {
                    Ok(_) => {}
                    Err(_) => break
                }
                if sleep_time > 0.0 {
                    tokio::time::sleep(Duration::from_nanos(sleep_time as u64)).await;
                }
            }
        });
    }

    pub fn send(&mut self, msg: FrameMessage) {
        if self.running {
            let tx = self.tx.clone();
            match msg {
                FrameMessage::Stop => self.running = false,
                _ => {}
            }
            tx.try_send(msg).expect("Send Error");
        }
    }
}
