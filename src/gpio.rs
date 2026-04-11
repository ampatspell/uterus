#[cfg(target_os = "linux")]
pub fn create_gpio() -> Result<(), Box<dyn Error>> {
    println!("create_gpio");

    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();
}

#[cfg(not(target_os = "linux"))]
pub fn create_gpio() {}
