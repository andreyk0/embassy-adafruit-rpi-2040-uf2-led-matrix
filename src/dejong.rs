use embedded_graphics::{pixelcolor::Rgb555, prelude::*};

const W: i32 = 32;
const H: i32 = 32;

// Peter de Jong attractor: x' = sin(a*y) - cos(b*x), y' = sin(c*x) - cos(d*y)
// Output is always in [-2, 2] x [-2, 2], covering the full display.
const A: f32 = 1.4;
const B: f32 = -2.3;
const C: f32 = 2.4;
const D: f32 = -2.1;

const TRAIL_LEN: usize = 48;

pub struct DeJong {
    x: f32,
    y: f32,
    trail: [(i8, i8, u8); TRAIL_LEN], // (px, py, color_idx 0-3)
    trail_idx: usize,
    last_quadrant: u8,
    color_idx: u8,
}

impl Default for DeJong {
    fn default() -> Self {
        Self::new()
    }
}

impl DeJong {
    pub fn new() -> Self {
        let init_quadrant = 0u8; // matches starting point (0.1, 0.2): x>0, y>0
        Self {
            x: 0.1,
            y: 0.2,
            trail: [(0, 0, 0); TRAIL_LEN],
            trail_idx: 0,
            last_quadrant: init_quadrant,
            color_idx: 0,
        }
    }

    pub fn step(&mut self) {
        let nx = libm::sinf(A * self.y) - libm::cosf(B * self.x);
        let ny = libm::sinf(C * self.x) - libm::cosf(D * self.y);

        self.x = nx;
        self.y = ny;

        // Map [-2, 2] → [0, 31]
        let px = ((self.x + 2.0) * (W as f32 / 4.0)) as i8;
        let py = ((self.y + 2.0) * (H as f32 / 4.0)) as i8;

        let quadrant = match (self.x > 0.0, self.y > 0.0) {
            (true, true) => 0u8,
            (false, true) => 1,
            (false, false) => 2,
            (true, false) => 3,
        };

        // Advance color each time the particle crosses a quadrant boundary
        if quadrant != self.last_quadrant {
            self.color_idx = (self.color_idx + 1) % 4;
            self.last_quadrant = quadrant;
        }

        self.trail[self.trail_idx] = (px, py, self.color_idx);
        self.trail_idx = (self.trail_idx + 1) % TRAIL_LEN;
    }

    pub fn draw<D>(&self, target: &mut D) -> Result<(), <D as DrawTarget>::Error>
    where
        D: DrawTarget<Color = Rgb555>,
    {
        let head_idx = (self.trail_idx + TRAIL_LEN - 1) % TRAIL_LEN;

        for i in 0..TRAIL_LEN {
            let idx = (self.trail_idx + i) % TRAIL_LEN;
            let (px, py, color_idx) = self.trail[idx];

            if px >= 0 && px < W as i8 && py >= 0 && py < H as i8 {
                let color = if idx == head_idx {
                    Rgb555::WHITE
                } else {
                    match color_idx {
                        0 => Rgb555::RED,
                        1 => Rgb555::GREEN,
                        2 => Rgb555::BLUE,
                        _ => Rgb555::YELLOW,
                    }
                };
                Pixel(Point::new(px as i32, py as i32), color).draw(target)?;
            }
        }
        Ok(())
    }
}
