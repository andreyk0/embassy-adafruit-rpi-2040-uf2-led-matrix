//! This example shows how to use USB (Universal Serial Bus) in the RP2040 chip.
//!
//! This creates the possibility to send logs to RTT or USB (if configured).

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Test pins
    let mut pins = [
        Output::new(p.PIN_7, Level::Low),
        Output::new(p.PIN_8, Level::Low),
    ];

    let mut counter = 0;
    loop {
        counter += 1;
        defmt::info!("Tick {}", counter);

        for p in pins.iter_mut() {
            p.toggle();
        }
        Timer::after_millis(100).await;
    }
}
