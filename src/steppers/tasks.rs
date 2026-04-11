use defmt::info;
use embassy_time::{Duration, Timer};
use esp_hal::gpio::Level;

use crate::steppers::{StepperPins, SteppersOptions};

//  Step C0 C1 C2 C3
//     1  1  0  1  0
//     2  0  1  1  0
//     3  0  1  0  1
//     4  1  0  0  1

pub type Step<const STEPS: usize> = [bool; STEPS];

pub struct Stepper<const STEPS: usize, const PINS: usize> {
    steps: [Step<PINS>; STEPS],
    pins: StepperPins<PINS>,
    step: usize,
}

impl<const STEPS: usize, const PINS: usize> Stepper<STEPS, PINS> {
    pub fn new(steps: [Step<PINS>; STEPS], pins: StepperPins<PINS>) -> Self {
        Self {
            steps,
            pins,
            step: 0,
        }
    }

    pub async fn step(&mut self) {
        info!("Step: {}", self.step);

        let step = self.steps[self.step];

        for (index, pin) in self.pins.iter_mut().enumerate() {
            let value = step[index];
            pin.set_level(if value { Level::High } else { Level::Low });
        }

        self.step += 1;
        if self.step > STEPS {
            self.step = 0;
        }

        Timer::after(Duration::from_micros(1000)).await;
    }
}

#[embassy_executor::task]
pub async fn steppers_task(pins: SteppersOptions) {
    let mut main = Stepper::<4, 4>::new(
        [
            [true, false, true, false],
            [false, true, true, false],
            [false, true, false, true],
            [true, false, false, true],
        ],
        pins.top,
    );

    loop {
        main.step().await;
    }
}
