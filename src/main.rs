extern crate sysfs_gpio;

use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};

fn main() {
    let led = Pin::new(3);

    println!("Exporting GPIO3");
    match led.with_exported(|| {
        println!("Setting GPIO3 direction to out at LOW");
        led.set_direction(Direction::Low)?;
        loop {
            println!("Setting GPIO3 to LOW");
            led.set_value(0)?;
            sleep(Duration::from_millis(500));

            println!("Setting GPIO3 to HIGH");
            led.set_value(1)?;
            sleep(Duration::from_millis(500));
        }
    }) {
        Ok(r) => r,
        Err(_) => std::process::exit(1),
    }
}
