use std::thread::park;

use crate::gpio::create_gpio;

mod gpio;

fn main() {
    println!("Uterus");

    create_gpio();

    park();
}
