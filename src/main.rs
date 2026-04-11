use std::{
    sync::{Arc, mpsc::channel},
    thread::park,
};

use crate::{channels::Message, gpio::create_gpio, tui::tui};

mod channels;
mod gpio;
mod tui;

fn main() {
    let (tx, rx) = channel::<Message>();
    let tx = Arc::new(tx);

    println!("Uterus");
    tui(&tx);
    create_gpio(rx);
    park();
}
