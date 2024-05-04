//! This example shows how to use USB (Universal Serial Bus) in the RP2040 chip.
//!
//! This creates the possibility to send log::info/warn/error/debug! to USB serial port.

#![no_std]
#![no_main]

use embassy_adafruit_rpi_2040_uf2_led_matrix::display::LedMatrixDisplay;
use embassy_adafruit_rpi_2040_uf2_led_matrix::matrix::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_time::Timer;
use embedded_graphics::pixelcolor::Rgb555;
use embedded_graphics::primitives::StyledDrawable;
use {defmt_rtt as _, panic_probe as _};

use embedded_graphics::{
    pixelcolor::{raw::RawU16, Rgb565, RgbColor},
    prelude::*,
    primitives::{Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::task]
async fn matrix_task(mut lm: LedMatrix<'static>, lmd: LedMatrixDisplay) {
    loop {
        lmd.run(&mut lm).await;
        Timer::after_millis(10).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let driver = Driver::new(p.USB, Irqs);

    spawner.spawn(logger_task(driver)).unwrap();

    let m_r1 = Output::new(p.PIN_8, Level::Low);
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
    let m_g1 = Output::new(p.PIN_7, Level::Low);

    let lm = LedMatrix::new(
        m_r1, m_r2, m_g1, m_g2, m_b1, m_b2, m_clk, m_lat, m_oe, m_a, m_b, m_c, m_d,
    );

    let mut lmd = LedMatrixDisplay::new();

    let style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb555::RED)
        .stroke_width(1)
        .fill_color(Rgb555::GREEN)
        .build();

    Timer::after_secs(3).await;

    Circle::with_center(Point::new(15, 15), 10)
        .draw_styled(&style, &mut lmd)
        .unwrap();

    spawner.spawn(matrix_task(lm, lmd)).unwrap();

    let mut counter = 0;
    loop {
        counter += 1;
        log::info!("Tick {}", counter);
        Timer::after_secs(1).await;
    }
}
