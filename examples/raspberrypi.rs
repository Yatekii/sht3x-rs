use embedded_hal::prelude::*;
use linux_embedded_hal::{Delay, I2cdev};
use sht3x::{Address, Repeatability, SHT3x};

fn main() {
    println!("Hello, SHT31!");

    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sht31 = SHT3x::new(dev, Address::Low);

    println!("Status raw: {:?}", sht31.status().unwrap());
    loop {
        sht31.start_measurement(Repeatability::High).unwrap();
        Delay.delay_ms(Repeatability::High.max_duration());
        let m = sht31.read_measurement().unwrap();
        println!("Temp: {:.2} Humidity: {:.2}", m.temperature, m.humidity);
    }
}
