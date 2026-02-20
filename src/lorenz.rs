use embedded_graphics::{pixelcolor::Rgb555, prelude::*};

const W: i32 = 32;
const H: i32 = 32;

// Standard Lorenz parameters
const SIGMA: f32 = 10.0;
const RHO: f32 = 28.0;
const BETA: f32 = 8.0 / 3.0;
const DT: f32 = 0.01;

pub struct Lorenz {
    x: f32,
    y: f32,
    z: f32,
    // Buffer for a fading effect tail: (x, y, z_normalized)
    trail: [(i8, i8, u8); 64],
    trail_idx: usize,
}

impl Default for Lorenz {
    fn default() -> Self {
        Self::new()
    }
}

impl Lorenz {
    pub fn new() -> Self {
        Self {
            x: 0.1,
            y: 0.0,
            z: 0.0,
            trail: [(-1, -1, 0); 64],
            trail_idx: 0,
        }
    }

    pub fn step(&mut self) {
        let dx = SIGMA * (self.y - self.x) * DT;
        let dy = (self.x * (RHO - self.z) - self.y) * DT;
        let dz = (self.x * self.y - BETA * self.z) * DT;

        self.x += dx;
        self.y += dy;
        self.z += dz;

        // Project 3D to 2D
        let px = ((self.x + 20.0) * (W as f32 / 40.0)) as i8;
        let py = ((self.y + 30.0) * (H as f32 / 60.0)) as i8;
        // Normalize Z (typically 0-50 range) to 0-31 for color mapping
        let pz = (self.z * (31.0 / 50.0)).max(0.0).min(31.0) as u8;

        self.trail[self.trail_idx] = (px, py, pz);
        self.trail_idx = (self.trail_idx + 1) % self.trail.len();
    }

    pub fn draw<D>(&self, target: &mut D) -> Result<(), <D as DrawTarget>::Error>
    where
        D: DrawTarget<Color = Rgb555>,
    {
        for i in 0..self.trail.len() {
            let idx = (self.trail_idx + i) % self.trail.len();
            let (px, py, pz) = self.trail[idx];

            if px >= 0 && px < W as i8 && py >= 0 && py < H as i8 {
                let is_head = i == self.trail.len() - 1;
                
                let color = if is_head {
                    Rgb555::WHITE // The leading spark
                } else {
                    // Map Z depth to a Blue (Low) -> Green -> Red (High) gradient
                    if pz < 10 {
                        Rgb555::BLUE
                    } else if pz < 20 {
                        Rgb555::CYAN
                    } else if pz < 25 {
                        Rgb555::GREEN
                    } else if pz < 28 {
                        Rgb555::YELLOW
                    } else {
                        Rgb555::RED
                    }
                };
                
                Pixel(Point::new(px as i32, py as i32), color).draw(target)?;
            }
        }
        Ok(())
    }
}
