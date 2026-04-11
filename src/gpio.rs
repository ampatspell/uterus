use crate::channels::Message;
use std::sync::mpsc::Receiver;

#[cfg(target_os = "linux")]
pub fn create_gpio(rx: Receiver<Message>) {
    use std::thread;

    use rpi_pal::gpio::Gpio;

    println!("create_gpio");

    let _ = thread::spawn(move || {
        use rpi_pal::gpio::{Error, Level, OutputPin};

        let create_pins = || -> Result<[OutputPin; 2], Error> {
            let pins = [
                Gpio::new()?.get(2)?.into_output_low(),
                Gpio::new()?.get(3)?.into_output_low(),
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

        let mut set = |l1: Level, l2: Level| {
            pins[0].write(l1);
            pins[1].write(l2);
        };

        loop {
            match rx.recv() {
                Ok(message) => match message {
                    Message::Direction { value } => match value {
                        0 => {
                            set(Level::High, Level::Low);
                        }
                        1 => {
                            set(Level::Low, Level::High);
                        }
                        _ => {}
                    },
                },
                Err(_) => {}
            }
        }
    });
}

#[cfg(not(target_os = "linux"))]
pub fn create_gpio(_: Arc<Receiver<Message>>) {}
