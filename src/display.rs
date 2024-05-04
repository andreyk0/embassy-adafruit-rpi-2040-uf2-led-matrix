//!
//! Matrix implementation of embedded graphics API
//!

use core::convert::TryInto;
use embedded_graphics::{
    pixelcolor::{Gray8, GrayColor, Rgb555},
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
};

use crate::matrix::LedMatrix;

use embassy_time::Timer;

pub struct LedMatrixDisplay {
    // 32 columns, color data shifted with clk pulses
    // 16 rows, addressed with a,b,c,d pins
    // r1,g1,b1 pins set pixel led on/off for 1/2 the matrix
    // r2,g2,b2 used for the other 1/2
    //
    // r1,g1,b1 lower nibble
    // r2,g2,b2 higher nibble
    //
    framebuffer: [u8; 32 * 16],
}

impl LedMatrixDisplay {
    pub fn new() -> Self {
        LedMatrixDisplay {
            framebuffer: [0u8; 32 * 16],
        }
    }

    /// Needs to be run in the loop to keep updating matrix
    pub async fn run(&self, lm: &mut LedMatrix<'_>) {
        for row in 0..16 {
            lm.oe(false);
            lm.addr(row);
            for column in 0..32 {
                lm.color(self.framebuffer[row as usize * 32 + column]);
                lm.clk();
            }
            lm.lat();
            lm.oe(true);
            Timer::after_millis(10).await;
        }
    }
}

impl Dimensions for LedMatrixDisplay {
    fn bounding_box(&self) -> Rectangle {
        Rectangle {
            top_left: Point { x: 0, y: 0 },
            size: Size {
                width: 32,
                height: 32,
            },
        }
    }
}

impl DrawTarget for LedMatrixDisplay {
    type Color = Rgb555;

    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(pcoord, color) in pixels.into_iter() {
            let x = pcoord.x.max(0).min(31) as usize;
            let y = pcoord.y.max(0).min(31) as usize;

            let c = ((color.r() & 0x1) << 2) | ((color.g() & 0x1) << 1) | (color.b() & 0x1);

            let (c, mask, y) = if y < 16 {
                (c, 0xf0, y)
            } else {
                (c << 4, 0x0f, y - 16)
            };

            let i = x + y * 32;
            self.framebuffer[i] &= mask;
            self.framebuffer[i] |= c;
        }

        Ok(())
    }
}
