use crate::channels::Message;
use std::sync::mpsc::Receiver;

#[cfg(target_os = "linux")]
pub fn create_gpio(rx: Receiver<Message>) {
    use std::thread;

    use rpi_pal::gpio::Gpio;

    println!("create_gpio");

    let _ = thread::spawn(move || {
        use rpi_pal::gpio::{Error, OutputPin};

        let create_pins = || -> Result<[OutputPin; 4], Error> {
            let pins = [
                Gpio::new()?.get(2)?.into_output(),
                Gpio::new()?.get(3)?.into_output(),
                Gpio::new()?.get(4)?.into_output(),
                Gpio::new()?.get(17)?.into_output(),
            ];

            Ok(pins)
        };

        let mut pins = match create_pins() {
            Ok(pins) => pins,
            Err(err) => {
                println!("Failed to create pins: {err:?}");
                return;
            }
        };

        pins.iter_mut().for_each(|pin| {
            pin.set_low();
        });

        loop {
            match rx.recv() {
                Ok(message) => match message {
                    Message::Key { value } => {
                        println!("Value: {value}");
                    }
                },
                Err(_) => {}
            }
        }
    });
}

#[cfg(not(target_os = "linux"))]
pub fn create_gpio(_: Arc<Receiver<Message>>) {}
