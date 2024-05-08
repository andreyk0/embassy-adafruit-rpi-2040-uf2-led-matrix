//! Render on a 2nd core, double buffer.

#![no_std]
#![no_main]

use core::ptr::addr_of;

use embassy_adafruit_rpi_2040_uf2_led_matrix::display::LedMatrixDisplay;
use embassy_adafruit_rpi_2040_uf2_led_matrix::gol::Gol;
use embassy_adafruit_rpi_2040_uf2_led_matrix::matrix::*;
use embassy_executor::Executor;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::multicore::{spawn_core1, Stack};
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_sync::once_lock::OnceLock;
use embassy_time::Timer;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

static mut CORE1_STACK: Stack<8192> = Stack::new();
static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();

static LMD1: StaticCell<LedMatrixDisplay> = StaticCell::new();
static LMD2: StaticCell<LedMatrixDisplay> = StaticCell::new();
// Ugh, this is ugly but works, going through a proper mutex type causes
// significant enough delays to be visible as a variation in brightness between
// scan lines
static mut LMD: Option<&'static mut LedMatrixDisplay> = None;
static LMD_READY: OnceLock<()> = OnceLock::new();

static GOL: StaticCell<Gol> = StaticCell::new();

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::task]
async fn matrix_task(mut lm: LedMatrix<'static>) {
    LMD_READY.get().await;
    log::info!("Starting matrix scans");

    // Run in a tight loop on a 2nd core, the whole core is dedicated to just
    // driing display waveforms
    loop {
        unsafe {
            if let Some(Some(lmd)) = addr_of!(LMD).as_ref() {
                lmd.run(&mut lm);
            }
        }
    }
}

#[embassy_executor::task]
async fn graphics_task() {
    Timer::after_millis(100).await; // let for usb serial to attach to see log messages

    // 2 bufferss, active/inactive
    let lmd1 = LMD1.init(LedMatrixDisplay::new());
    unsafe {
        LMD.replace(lmd1);
    }
    let mut lmd = LMD2.init(LedMatrixDisplay::new());
    LMD_READY.init(()).unwrap();

    let gol = GOL.init(Gol::new());
    gol.randomize();
    gol.glider();

    log::info!("Starting matrix scans");

    loop {
        // draw onto inactive, flip
        gol.draw(lmd).unwrap();
        unsafe {
            lmd = LMD.replace(lmd).unwrap();
        }
        gol.step();

        Timer::after_millis(100).await;
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_rp::init(Default::default());
    let driver = Driver::new(p.USB, Irqs);

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

    spawn_core1(
        p.CORE1,
        unsafe { &mut *core::ptr::addr_of_mut!(CORE1_STACK) },
        move || {
            let executor1 = EXECUTOR1.init(Executor::new());
            executor1.run(|spawner| spawner.spawn(matrix_task(lm)).unwrap());
        },
    );

    let executor0 = EXECUTOR0.init(Executor::new());
    executor0.run(|spawner| {
        spawner.spawn(logger_task(driver)).unwrap();
        spawner.spawn(graphics_task()).unwrap();
    });
}
