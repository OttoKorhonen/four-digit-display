#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::prelude::*;
use esp_hal::gpio::{OutputOpenDrain, Level, Pull};
use tm1637::tm1637::Tm1637;

#[entry]
fn main() -> ! {
    let _peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    let mut dio = OutputOpenDrain::new(_peripherals.GPIO1, Level::Low, Pull::Up);
    let mut scl = OutputOpenDrain::new(_peripherals.GPIO0, Level::Low, Pull::Up);

    esp_println::logger::init_logger_from_env();

    let delay = Delay::new();
    let mut display = Tm1637::new(&mut dio, &mut scl, delay);

    display.init();

    
    loop {

        for i in 0..=1000 {
            display.write(i);
            delay.delay_millis(1000);
        }
        
        delay.delay(500.millis());
    }
}
