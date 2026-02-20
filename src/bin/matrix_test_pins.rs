//! This example shows how to use USB (Universal Serial Bus) in the RP2040 chip.
//!
//! This creates the possibility to send logs to RTT or USB (if configured).

#![no_std]
#![no_main]

use embassy_adafruit_rpi_2040_uf2_led_matrix::matrix::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
async fn matrix_task(mut lm: LedMatrix<'static>) {
    let c = [
        0b0001_0001,
        0b0010_0010,
        0b0100_0100,
        0b0101_0101,
        0b0110_0110,
        0b0111_0111,
        0b0011_0011,
        0b0010_0111,
        0b0110_0110,
        0b0111_0111,
        0b0011_0011,
        0b0101_0101,
    ];
    let mut cnt = 0u16;

    loop {
        let a = (cnt % 17) as u8;
        lm.addr(a);

        lm.color(c[cnt as usize % c.len()]);
        cnt = cnt.wrapping_add(1);

        lm.oe(false);
        for _ in 0..32 {
            lm.clk();
        }
        lm.lat();

        lm.oe(true);

        Timer::after_millis(10).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

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

    spawner.spawn(matrix_task(lm)).unwrap();

    let mut counter = 0;
    loop {
        counter += 1;
        defmt::info!("Tick {}", counter);
        Timer::after_secs(8).await;
    }
}
