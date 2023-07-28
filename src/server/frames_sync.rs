use crate::server::messages::Message;
use actix::Recipient;
use bytes::{BufMut, Bytes, BytesMut};
use std::time::{Duration, Instant};
use tokio::sync::mpsc::{Receiver, Sender};

#[derive(Debug, PartialEq)]
pub enum FrameMessage {
    Stop,
    KeyBuffer(Bytes),
    Frame,
}

#[derive(Debug)]
pub struct Frames {
    running: bool,
    tx: Sender<FrameMessage>,
    rx: Option<Receiver<FrameMessage>>,
}

impl Default for Frames {
    fn default() -> Self {
        Self::new()
    }
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
            return;
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
            loop {
                if let Some(msg) = rx.recv().await {
                    match msg {
                        FrameMessage::Frame => {
                            for addr in addrs.iter() {
                                addr.do_send(Message::Binary(Bytes::from(frame_buffer.clone())))
                            }
                            frame_buffer.clear();
                        }
                        FrameMessage::KeyBuffer(bytes) => frame_buffer.put(bytes),
                        FrameMessage::Stop => break,
                    }
                }
            }
        });
        let tx = self.tx.clone();
        tokio::spawn(async move {
            let sleep_time = Duration::from_secs_f64(1.0 / 60.0);
            let mut next_game_tick = Duration::from_secs(0);
            let now = Instant::now();
            loop {
                next_game_tick += sleep_time;
                match tx.send(FrameMessage::Frame).await {
                    Ok(_) => {
                        let interval = now.elapsed();
                        if next_game_tick > interval {
                            tokio::time::sleep(next_game_tick - interval).await;
                        }
                    }
                    Err(_) => break,
                }
            }
        });
    }

    pub fn send(&mut self, msg: FrameMessage) {
        if self.running {
            if FrameMessage::Stop == msg {
                self.running = false;
            }
            self.tx.try_send(msg).expect("Send Error");
        }
    }
}
