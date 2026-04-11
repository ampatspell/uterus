use crate::channels::Message;
use std::{sync::mpsc::Sender, thread, time::Duration};

pub fn loop_task(tx: &Sender<Message>) {
    let tx = tx.clone();
    let _ = thread::spawn(move || {
        loop {
            tx.send(Message::Direction { value: 0 }).unwrap();
            thread::sleep(Duration::from_secs(5));
            tx.send(Message::Direction { value: 1 }).unwrap();
            thread::sleep(Duration::from_secs(5));
        }
    });
}
