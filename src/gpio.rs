use crate::channels::Direction;
use crate::channels::Limit;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

#[cfg(target_os = "linux")]
use std::{
    thread::{self},
    time::Duration,
};

#[cfg(target_os = "linux")]
use rpi_pal::gpio::{Gpio, InputPin, Trigger};

#[cfg(target_os = "linux")]
fn create_gpio_outputs(rx: Receiver<Direction>) {
    use rpi_pal::gpio::Gpio;
    use std::thread;

    let _ = thread::spawn(move || {
        use rpi_pal::{
            gpio::{Error, Level, OutputPin},
            pwm::Pwm,
        };
        use std::cell::OnceCell;

        thread_local! {
            static PWM: OnceCell<Pwm> = OnceCell::new();
        }

        let pwm = Pwm::new(rpi_pal::pwm::Channel::Pwm0).unwrap();
        pwm.set_frequency(100.0, 0.5).unwrap();
        pwm.enable().unwrap();

        PWM.with(|cell| cell.set(pwm).unwrap());

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
                Ok(direction) => match direction {
                    Direction::Forward => {
                        set(Level::High, Level::Low);
                    }
                    Direction::Backward => {
                        set(Level::Low, Level::High);
                    }
                    Direction::Stop => {
                        set(Level::Low, Level::Low);
                    }
                },
                Err(_) => {}
            }
        }
    });
}

#[cfg(target_os = "linux")]
fn create_gpio_inputs(limit_tx: Sender<Limit>) {
    let _ = thread::spawn(move || {
        use std::cell::OnceCell;

        let mut left = Gpio::new().unwrap().get(17).unwrap().into_input_pullup();
        let mut right = Gpio::new().unwrap().get(27).unwrap().into_input_pullup();

        thread_local! {
            static PINS: OnceCell<[InputPin;2]> = OnceCell::new();
        }

        let tx = limit_tx.clone();

        left.set_async_interrupt(
            Trigger::FallingEdge,
            Some(Duration::from_millis(5)),
            move |_| {
                tx.send(Limit::Backward).unwrap();
            },
        )
        .unwrap();

        let tx = limit_tx.clone();

        right
            .set_async_interrupt(
                Trigger::FallingEdge,
                Some(Duration::from_millis(5)),
                move |_| {
                    tx.send(Limit::Forward).unwrap();
                },
            )
            .unwrap();

        PINS.try_with(|pins| {
            pins.set([left, right]).unwrap();
        })
        .unwrap();

        thread::park();
    });
}

#[cfg(target_os = "linux")]
pub fn create_gpio(direction_rx: Receiver<Direction>, limit_tx: Sender<Limit>) {
    create_gpio_outputs(direction_rx);
    create_gpio_inputs(limit_tx);
}

#[cfg(not(target_os = "linux"))]
pub fn create_gpio(_: Receiver<Direction>, _: Sender<Limit>) {}
