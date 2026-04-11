#[cfg(target_os = "linux")]
pub fn create_gpio() {
    println!("Create GPIO");
}

#[cfg(not(target_os = "linux"))]
pub fn create_gpio() {}
