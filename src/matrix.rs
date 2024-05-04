/// P6-3528-32*32-16S-HL1.1 led matrix
/// <https://cdn-learn.adafruit.com/downloads/pdf/32x16-32x32-rgb-led-matrix.pdf>
use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_time::Timer;

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

impl LedMatrix<'_> {
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

    pub fn color(&mut self, c: u8) {
        let r1l = Level::from(c & 0b100000 != 0);
        let r2l = Level::from(c & 0b010000 != 0);

        let g1l = Level::from(c & 0b001000 != 0);
        let g2l = Level::from(c & 0b000100 != 0);

        let b1l = Level::from(c & 0b000010 != 0);
        let b2l = Level::from(c & 0b000001 != 0);

        self.r1.set_level(r1l);
        self.r2.set_level(r2l);

        self.g1.set_level(g1l);
        self.g2.set_level(g2l);

        self.b1.set_level(b1l);
        self.b2.set_level(b2l);
    }

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

    pub fn oe(&mut self, en: bool) {
        self.oe.set_level((!en).into());
    }

    pub async fn clk(&mut self) {
        self.clk.set_high();
        Timer::after_micros(10).await;
        self.clk.set_low();
        Timer::after_micros(10).await;
    }

    pub async fn lat(&mut self) {
        self.lat.set_high();
        Timer::after_micros(10).await;
        self.lat.set_low();
        Timer::after_micros(10).await;
    }
}