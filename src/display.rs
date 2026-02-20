//!
//! Matrix implementation of embedded graphics API
//!

use embedded_graphics::{pixelcolor::Rgb555, prelude::*, primitives::Rectangle};

use crate::matrix::LedMatrix;

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

const PULSE_DELAY_CYCLES: u32 = 512;

impl LedMatrixDisplay {
    pub fn new() -> Self {
        LedMatrixDisplay {
            framebuffer: [0u8; 32 * 16],
        }
    }

    pub fn clear(&mut self) {
        self.framebuffer.fill(0);
    }

    /// Needs to be run in the loop to keep updating matrix
    pub fn run(&self, lm: &mut LedMatrix<'_>) {
        for row in 0..16 {
            lm.oe(false);
            lm.addr(row);
            for column in 0..32 {
                lm.color(self.framebuffer[row as usize * 32 + column]);
                lm.clk();
            }
            lm.lat();
            lm.oe(true);
            cortex_m::asm::delay(PULSE_DELAY_CYCLES);
        }
    }
}

impl Default for LedMatrixDisplay {
    fn default() -> Self {
        LedMatrixDisplay::new()
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
        for Pixel(pcoord, color) in pixels {
            let x = pcoord.x;
            let y = pcoord.y;

            if x < 0 || x >= 32 || y < 0 || y >= 32 {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            let r = if color.r() > 0 { 1 } else { 0 };
            let g = if color.g() > 0 { 1 } else { 0 };
            let b = if color.b() > 0 { 1 } else { 0 };

            let c = (r << 2) | (g << 1) | b;

            if y < 16 {
                let i = x + y * 32;
                self.framebuffer[i] = (self.framebuffer[i] & 0xf0) | c;
            } else {
                let i = x + (y - 16) * 32;
                self.framebuffer[i] = (self.framebuffer[i] & 0x0f) | (c << 4);
            }
        }

        Ok(())
    }
}
