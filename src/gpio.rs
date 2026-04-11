#[cfg(target_os = "linux")]
pub fn create_gpio() {
    use std::thread;

    use rpi_pal::gpio::Gpio;

    println!("create_gpio");

    let handle = thread::spawn(move || {
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

        let pins = match create_pins() {
            Ok(pins) => pins,
            Err(err) => {
                println!("Failed to create pins: {err:?}");
                return;
            }
        };

        {
            let mut pins = pins;
            let one = &mut pins[0];
            one.set_high();
        }
    });

    handle.join().unwrap();
}

#[cfg(not(target_os = "linux"))]
pub fn create_gpio() {}
