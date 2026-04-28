use crate::channels::{get_inverse, Direction, Limit};
use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
};

pub fn loop_task(tx: Sender<Direction>, rx: Receiver<Limit>) {
    let tx = tx.clone();
    let _ = thread::spawn(move || {
        let mut direction = Direction::Backward;
        loop {
            println!("Send {direction:?}");
            tx.send(direction).unwrap();
            match rx.recv() {
                Ok(limit) => {
                    let inverse = get_inverse(limit);
                    if inverse != direction {
                        direction = inverse
                    }
                }
                Err(err) => {
                    dbg!(err);
                }
            }
        }
    });
}
