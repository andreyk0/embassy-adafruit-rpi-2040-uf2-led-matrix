use embedded_graphics::{pixelcolor::Rgb555, prelude::*};
use embassy_rp::clocks::RoscRng;

const W: usize = 32;
const H: usize = 32;
const STATES: u8 = 14;
const THRESHOLD: u8 = 3;

pub struct Cca {
    states: [u8; W * H],
    next_states: [u8; W * H],
}

impl Default for Cca {
    fn default() -> Self {
        Self::new()
    }
}

impl Cca {
    pub fn new() -> Self {
        let mut cca = Self {
            states: [0; W * H],
            next_states: [0; W * H],
        };
        cca.randomize();
        cca
    }

    pub fn randomize(&mut self) {
        let mut rng = RoscRng;
        defmt::info!("Randomizing CCA states...");
        let mut sum = 0u32;
        for i in 0..self.states.len() {
            let s = (rng.next_u32() % STATES as u32) as u8;
            self.states[i] = s;
            sum += s as u32;
        }
        defmt::info!("CCA Randomized. Total state sum: {}", sum);
    }

    pub fn step(&mut self) {
        let mut changed = 0;
        for y in 0..H {
            for x in 0..W {
                let i = y * W + x;
                let current_state = self.states[i];
                let next_target = (current_state + 1) % STATES;
                let mut count = 0;

                // Check 8 neighbors with wrapping
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let nx = (x as i32 + dx).rem_euclid(W as i32) as usize;
                        let ny = (y as i32 + dy).rem_euclid(H as i32) as usize;

                        if self.states[ny * W + nx] == next_target {
                            count += 1;
                        }
                    }
                }

                if count >= THRESHOLD {
                    self.next_states[i] = next_target;
                    changed += 1;
                } else {
                    self.next_states[i] = current_state;
                }
            }
        }
        self.states.copy_from_slice(&self.next_states);
        if changed == 0 {
            // If nothing changed, it means we reached a steady state or randomization failed.
            // Let's re-randomize one pixel to kickstart it if it's dead.
            let i = (RoscRng.next_u32() % (W * H) as u32) as usize;
            self.states[i] = (self.states[i] + 1) % STATES;
        }
    }

    pub fn draw<D>(&self, target: &mut D) -> Result<(), <D as DrawTarget>::Error>
    where
        D: DrawTarget<Color = Rgb555>,
    {
        let it = self.states.iter().enumerate().map(|(i, &state)| {
            let x = (i % W) as i32;
            let y = (i / W) as i32;
            
            // Map 14 states to distinct colors
            let color = match state {
                0 => Rgb555::RED,
                1 => Rgb555::GREEN,
                2 => Rgb555::BLUE,
                3 => Rgb555::YELLOW,
                4 => Rgb555::CYAN,
                5 => Rgb555::MAGENTA,
                6 => Rgb555::WHITE,
                7 => Rgb555::new(31, 15, 0),   // Orange
                8 => Rgb555::new(15, 31, 0),   // Lime
                9 => Rgb555::new(0, 31, 15),   // Teal
                10 => Rgb555::new(15, 0, 31),  // Purple
                11 => Rgb555::new(31, 0, 15),  // Pink
                12 => Rgb555::new(15, 15, 31), // Sky Blue
                _ => Rgb555::new(31, 31, 15),  // Pale Yellow
            };
            
            Pixel(Point::new(x, y), color)
        });

        target.draw_iter(it)
    }
}
