use embassy_executor::Spawner;
use esp_hal::gpio::Output;

use crate::steppers::tasks::steppers_task;

pub mod tasks;

pub type StepperPins<const PINS: usize> = [Output<'static>; PINS];

pub struct SteppersOptions {
    pub top: StepperPins<4>,
}

pub fn spawn_steppers_task(spawner: &Spawner, pins: SteppersOptions) {
    spawner.spawn(steppers_task(pins)).unwrap();
}
