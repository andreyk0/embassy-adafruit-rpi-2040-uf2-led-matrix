//! <https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life>

use core::u32;
use embedded_graphics::{pixelcolor::Rgb555, prelude::*};
use rand::RngCore;

const W: usize = 32;
const H: usize = 32;

/// 32x32 wraparound + colors + spawn when empty for a long time
pub struct Gol {
    // Empty point ages.
    // Age 0 represents a non-empty point, it's "alive".
    ages: [u16; W * H],
}

impl Default for Gol {
    fn default() -> Self {
        Self::new()
    }
}

impl Gol {
    pub fn new() -> Self {
        Self { ages: [1; W * H] }
    }

    pub fn randomize(&mut self) {
        for x in 0..W {
            for y in 0..H {
                let a = if embassy_rp::clocks::RoscRng.next_u32() > u32::MAX / 2 {
                    0
                } else {
                    1
                };
                self.ages[Self::point_idx(&Point::new(x as i32, y as i32))] = a;
            }
        }
    }

    pub fn glider(&mut self) {
        for (x, y) in [(1, 1), (1, 3), (2, 2), (2, 3), (3, 2)] {
            self.ages[Self::point_idx(&Point::new(x, y))] = 0;
        }
    }

    pub fn step(&mut self) {
        let mut neighbors = [0u8; W * H];

        for x in 0..W {
            for y in 0..H {
                let p = Point::new(x as i32, y as i32);
                neighbors[Self::point_idx(&p)] = self.num_neighbors(&p);
            }
        }

        for x in 0..W {
            for y in 0..H {
                let p = Point::new(x as i32, y as i32);
                let i = Self::point_idx(&p);

                match neighbors[i] {
                    2 if self.ages[i] == 0 => {
                        self.ages[i] = 0;
                    }
                    3 => {
                        self.ages[i] = 0;
                    }
                    _ => {
                        // spawn new life when space is empty for a long time
                        let rn = embassy_rp::clocks::RoscRng.next_u32();
                        if self.ages[i] as u32 > rn {
                            self.ages[i] = 0;
                        } else {
                            self.ages[i] = self.ages[i].saturating_add(1);
                        }
                    }
                }
            }
        }
    }

    pub fn draw<D>(&self, target: &mut D) -> Result<(), <D as DrawTarget>::Error>
    where
        D: DrawTarget<Color = Rgb555>,
    {
        let it = (0..self.ages.len()).map(|i| {
            let y = i / 32;
            let x = i % 32;
            let p = Point::new(x as i32, y as i32);
            let epa = self.empty_point_age(&p);
            let c = if epa == 0 {
                match self.num_neighbors(&p) {
                    1 => Rgb555::BLUE,
                    2 => Rgb555::GREEN,
                    3 => Rgb555::RED,
                    _ => Rgb555::BLUE,
                }
            } else {
                Rgb555::BLACK
            };

            Pixel(p, c)
        });

        target.draw_iter(it)
    }

    fn num_neighbors(&self, Point { x, y }: &Point) -> u8 {
        let mut n = 0;

        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                let dp = Point::new(x + dx, y + dy);
                if self.empty_point_age(&dp) == 0 && !(dx == 0 && dy == 0) {
                    n += 1;
                }
            }
        }

        n
    }

    #[inline]
    fn empty_point_age(&self, p: &Point) -> u16 {
        self.ages[Self::point_idx(p)]
    }

    fn point_idx(Point { x, y }: &Point) -> usize {
        let x = (32 + x) % 32;
        let y = (32 + y) % 32;
        (x + 32 * y) as usize
    }
}
