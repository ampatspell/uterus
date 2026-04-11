#[cfg(target_os = "linux")]
pub fn create_gpio() {
    use std::thread;

    use rpi_pal::gpio::Gpio;

    println!("create_gpio");

    let handle = thread::spawn(move || {
        use rpi_pal::gpio::{Error, OutputPin};

        struct Pins {
            one: OutputPin,
        }

        let create_pins = || -> Result<Pins, Error> {
            let one = Gpio::new()?.get(4)?.into_output();

            Ok(Pins { one })
        };

        let Pins { one } = match create_pins() {
            Ok(pins) => pins,
            Err(err) => {
                println!("Failed to create pins: {err:?}");
                return;
            }
        };

        {
            let mut one = one;
            one.set_high();
        }
    });

    handle.join().unwrap();
}

#[cfg(not(target_os = "linux"))]
pub fn create_gpio() {}
