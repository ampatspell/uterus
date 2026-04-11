use crate::channels::Message;
use console::Term;
use std::{sync::mpsc::Sender, thread};

pub fn tui(tx: &Sender<Message>) {
    let tx = tx.clone();
    let _ = thread::spawn(move || {
        let term = Term::stdout();
        loop {
            let next = term.read_char();
            match next {
                Ok(value) => {
                    tx.send(Message::Key { value }).unwrap();
                }
                Err(err) => {
                    dbg!(err);
                }
            }
        }
    });
}
