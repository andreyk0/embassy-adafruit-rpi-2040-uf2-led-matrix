//! P6-3528-32*32-16S-HL1.1 led matrix
//! <https://cdn-learn.adafruit.com/downloads/pdf/32x16-32x32-rgb-led-matrix.pdf>

use embassy_rp::gpio::{Level, Output};

pub struct LedMatrix<'a> {
    r1: Output<'a>,
    r2: Output<'a>,
    g1: Output<'a>,
    g2: Output<'a>,
    b1: Output<'a>,
    b2: Output<'a>,

    clk: Output<'a>,
    lat: Output<'a>,
    oe: Output<'a>,

    a: Output<'a>,
    b: Output<'a>,
    c: Output<'a>,
    d: Output<'a>,
}

const PULSE_DELAY_CYCLES: u32 = 4;

impl LedMatrix<'_> {
    #![allow(clippy::too_many_arguments)]
    pub fn new<'a>(
        r1: Output<'a>,
        r2: Output<'a>,
        g1: Output<'a>,
        g2: Output<'a>,
        b1: Output<'a>,
        b2: Output<'a>,

        clk: Output<'a>,
        lat: Output<'a>,
        oe: Output<'a>,

        a: Output<'a>,
        b: Output<'a>,
        c: Output<'a>,
        d: Output<'a>,
    ) -> LedMatrix<'a> {
        LedMatrix {
            r1,
            r2,
            g1,
            g2,
            b1,
            b2,
            clk,
            lat,
            oe,
            a,
            b,
            c,
            d,
        }
    }

    /// Set colors.
    /// 0 r2 g2 b2 0 r1 g1 b1
    pub fn color(&mut self, c: u8) {
        let r1l = Level::from(c & 0b0000_0100 != 0);
        let g1l = Level::from(c & 0b0000_0010 != 0);
        let b1l = Level::from(c & 0b0000_0001 != 0);

        let r2l = Level::from(c & 0b0100_0000 != 0);
        let g2l = Level::from(c & 0b0010_0000 != 0);
        let b2l = Level::from(c & 0b0001_0000 != 0);

        self.r1.set_level(r1l);
        self.r2.set_level(r2l);

        self.g1.set_level(g1l);
        self.g2.set_level(g2l);

        self.b1.set_level(b1l);
        self.b2.set_level(b2l);
    }

    /// Set row address 0-15
    pub fn addr(&mut self, a: u8) {
        let al = Level::from(a & 0b0001 != 0);
        let bl = Level::from(a & 0b0010 != 0);
        let cl = Level::from(a & 0b0100 != 0);
        let dl = Level::from(a & 0b1000 != 0);

        self.a.set_level(al);
        self.b.set_level(bl);
        self.c.set_level(cl);
        self.d.set_level(dl);
    }

    /// Enable/disable output
    pub fn oe(&mut self, en: bool) {
        self.oe.set_level((!en).into());
    }

    /// Send a clock pulse
    pub fn clk(&mut self) {
        self.clk.set_high();
        cortex_m::asm::delay(PULSE_DELAY_CYCLES);
        self.clk.set_low();
        cortex_m::asm::delay(PULSE_DELAY_CYCLES);
    }

    /// Latch output
    pub fn lat(&mut self) {
        self.lat.set_high();
        cortex_m::asm::delay(PULSE_DELAY_CYCLES);
        self.lat.set_low();
        cortex_m::asm::delay(PULSE_DELAY_CYCLES);
    }
}
