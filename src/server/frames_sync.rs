use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;
use chrono::Local;

#[derive(Debug)]
pub enum FrameMessage {
    Stop,
}

struct Frames {
    running: bool,
    tx: Sender<FrameMessage>,
    rx: Option<Receiver<FrameMessage>>
}

impl Frames {
    pub fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        Self {
            running: false,
            tx,
            rx: Some(rx),
        }
    }

    pub fn start(&mut self) {
        let mut rx = match self.rx.take() {
            Some(rx) => rx,
            None => {
                let (tx, rx) = std::sync::mpsc::channel();
                self.tx = tx;
                rx
            }
        };
        tokio::spawn(async move {
            let mut next_game_tick = Local::now().timestamp_nanos() as f64;
            let mut fps = 0;
            let mut p = Local::now().timestamp_millis();
            loop {
                next_game_tick += MS_PER_UPDATE;
                let sleep_time = next_game_tick - Local::now().timestamp_nanos() as f64;
                let current = Local::now().timestamp_millis();
                if current - p >= 1000 {
                    println!("fps = {}", fps);
                    fps = 0;
                    p = current;
                }
                fps += 1;
                if let Ok(msg) = rx.try_recv() {
                    match msg {
                        FrameMessage::Stop => {
                            break
                        }
                    }
                }
                if sleep_time > 0.0 {
                    tokio::time::sleep(Duration::from_nanos(sleep_time as u64)).await;
                }
            }

        });
    }

    pub fn send(&self, msg: FrameMessage) {
        self.tx.send(msg).expect("Send Error");
    }
}
