//! This example shows how to use USB (Universal Serial Bus) in the RP2040 chip.
//!
//! This creates the possibility to send log::info/warn/error/debug! to USB serial port.

#![no_std]
#![no_main]

use embassy_adafruit_rpi_2040_uf2_led_matrix::matrix::*;
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

#[embassy_executor::task]
async fn matrix_task(mut lm: LedMatrix<'static>) {
    lm.addr(0);

    let c = [0b01, 0b0100, 0b010000];
    let mut cnt = 0u8;

    loop {
        lm.color(c[cnt as usize % c.len()]);
        cnt = cnt.wrapping_add(1);

        lm.oe(false);

        lm.clk();
        lm.clk();
        lm.clk();
        lm.lat();

        lm.oe(true);

        Timer::after_secs(1).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let driver = Driver::new(p.USB, Irqs);

    spawner.spawn(logger_task(driver)).unwrap();

    let m_r1 = Output::new(p.PIN_6, Level::Low);
    let m_b1 = Output::new(p.PIN_9, Level::Low);
    let m_r2 = Output::new(p.PIN_11, Level::Low);
    let m_b2 = Output::new(p.PIN_12, Level::Low);
    let m_a = Output::new(p.PIN_25, Level::Low);
    let m_c = Output::new(p.PIN_29, Level::Low);
    let m_clk = Output::new(p.PIN_13, Level::Low);
    let m_oe = Output::new(p.PIN_0, Level::Low);

    let m_lat = Output::new(p.PIN_1, Level::Low);
    let m_d = Output::new(p.PIN_28, Level::Low);
    let m_b = Output::new(p.PIN_24, Level::Low);
    let m_g2 = Output::new(p.PIN_10, Level::Low);
    let m_g1 = Output::new(p.PIN_5, Level::Low);

    let lm = LedMatrix::new(
        m_r1, m_r2, m_g1, m_g2, m_b1, m_b2, m_clk, m_lat, m_oe, m_a, m_b, m_c, m_d,
    );

    spawner.spawn(matrix_task(lm)).unwrap();

    let mut counter = 0;
    loop {
        counter += 1;
        log::info!("Tick {}", counter);
        Timer::after_secs(1).await;
    }
}
