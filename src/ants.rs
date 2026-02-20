use embedded_graphics::{pixelcolor::Rgb555, prelude::*};

const W: usize = 32;
const H: usize = 32;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}

struct Ant {
    x: i32,
    y: i32,
    dir: Direction,
    color: Rgb555,
}

pub struct Ants {
    grid: [u8; W * H],
    ants: [Ant; 3],
}

impl Default for Ants {
    fn default() -> Self {
        Self::new()
    }
}

impl Ants {
    pub fn new() -> Self {
        Self {
            grid: [0; W * H],
            ants: [
                Ant { x: 10, y: 10, dir: Direction::Up, color: Rgb555::RED },
                Ant { x: 20, y: 10, dir: Direction::Right, color: Rgb555::GREEN },
                Ant { x: 15, y: 20, dir: Direction::Down, color: Rgb555::BLUE },
            ],
        }
    }

    pub fn step(&mut self) {
        for ant in self.ants.iter_mut() {
            let idx = (ant.y as usize * W + ant.x as usize) % (W * H);
            let cell = self.grid[idx];

            if cell == 0 {
                ant.dir = ant.dir.turn_right();
                self.grid[idx] = 1; // Mark as "on"
            } else {
                ant.dir = ant.dir.turn_left();
                self.grid[idx] = 0; // Mark as "off"
            }

            // Move ant
            match ant.dir {
                Direction::Up => ant.y -= 1,
                Direction::Right => ant.x += 1,
                Direction::Down => ant.y += 1,
                Direction::Left => ant.x -= 1,
            }

            // Wrap around
            ant.x = (ant.x + W as i32) % W as i32;
            ant.y = (ant.y + H as i32) % H as i32;
        }
    }

    pub fn draw<D>(&self, target: &mut D) -> Result<(), <D as DrawTarget>::Error>
    where
        D: DrawTarget<Color = Rgb555>,
    {
        // Draw grid
        for i in 0..self.grid.len() {
            if self.grid[i] != 0 {
                let x = (i % W) as i32;
                let y = (i / W) as i32;
                Pixel(Point::new(x, y), Rgb555::WHITE).draw(target)?;
            }
        }

        // Draw ants with their colors
        for ant in self.ants.iter() {
            Pixel(Point::new(ant.x, ant.y), ant.color).draw(target)?;
        }

        Ok(())
    }
}
