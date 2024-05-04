//! This example shows how to use USB (Universal Serial Bus) in the RP2040 chip.
//!
//! This creates the possibility to send log::info/warn/error/debug! to USB serial port.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let driver = Driver::new(p.USB, Irqs);

    // Test pins
    let mut pins = [
        // Output::new(p.PIN_0, Level::Low),
        // Output::new(p.PIN_1, Level::Low),
        // Output::new(p.PIN_2, Level::Low),
        // Output::new(p.PIN_3, Level::Low),
        // Output::new(p.PIN_4, Level::Low),
        // Output::new(p.PIN_5, Level::Low),
        // Output::new(p.PIN_6, Level::High),
        Output::new(p.PIN_7, Level::Low),
        Output::new(p.PIN_8, Level::Low),
        // Output::new(p.PIN_9, Level::Low),
        // Output::new(p.PIN_10, Level::Low),
        // Output::new(p.PIN_11, Level::Low),
        // Output::new(p.PIN_12, Level::Low),
        // Output::new(p.PIN_13, Level::Low),
        // Output::new(p.PIN_24, Level::Low),
        // Output::new(p.PIN_25, Level::Low),
        // Output::new(p.PIN_26, Level::Low),
        // Output::new(p.PIN_27, Level::Low),
        // Output::new(p.PIN_28, Level::Low),
        // Output::new(p.PIN_29, Level::Low),
    ];

    spawner.spawn(logger_task(driver)).unwrap();

    let mut counter = 0;
    loop {
        counter += 1;
        log::info!("Tick {}", counter);

        for p in pins.iter_mut() {
            p.toggle();
        }
        Timer::after_millis(100).await;
    }
}
