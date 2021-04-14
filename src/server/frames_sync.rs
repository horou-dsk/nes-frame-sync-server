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
        let (tx, rx) = tokio::sync::mpsc::channel(32);
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
                let (tx, rx) = tokio::sync::mpsc::channel(32);
                self.tx = tx;
                rx
            }
        };
        tokio::spawn(async move {
            let mut frame_buffer = BytesMut::new();
            loop {
                if let Some(msg) = rx.recv().await {
                    match msg {
                        FrameMessage::Stop => {
                            break
                        },
                        FrameMessage::Frame => {
                            for addr in addrs.iter() {
                                addr.do_send(Message::Binary(Bytes::from(frame_buffer.clone()))).unwrap();
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
            let mut fps = 0;
            let mut p = Local::now().timestamp_millis();
            loop {
                next_game_tick += MS_PER_UPDATE;
                let sleep_time = next_game_tick - Local::now().timestamp_nanos() as f64;
                let current = Local::now().timestamp_millis();
                if current - p >= 1000 {
                    // println!("fps = {}", fps);
                    fps = 0;
                    p = current;
                }
                fps += 1;
                tx.send(FrameMessage::Frame).await.expect("Frame Error");
                if sleep_time > 0.0 {
                    tokio::time::sleep(Duration::from_nanos(sleep_time as u64)).await;
                }
            }
        });
    }

    pub fn send(&self, msg: FrameMessage) {
        let tx = self.tx.clone();
        tokio::spawn(async move {
            tx.send(msg).await.expect("Send Error");
        });
    }
}
