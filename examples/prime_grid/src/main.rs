use bitvec::prelude::*;
use jotter::Image;

enum Direction {
    Right,
    Up,
    Left,
    Down,
}

struct PositionIterator {
    x: usize,
    y: usize,
    step: usize,
    step_count: usize,
    direction: Direction,
}

impl PositionIterator {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            step: 0,
            step_count: 1,
            direction: Direction::Right,
        }
    }

    pub fn next(&mut self) -> (usize, usize) {
        let out = (self.x, self.y);
        match self.direction {
            Direction::Right => self.x += 1,
            Direction::Up => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Down => self.y -= 1,
        }
        self.step += 1;
        if self.step == self.step_count {
            self.step_count += match self.direction {
                Direction::Up | Direction::Down => 1,
                _ => 0,
            };
            self.direction = match self.direction {
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
            };
            self.step = 0;
        }
        out
    }
}

fn main() -> Result<(), jotter::Error> {
    const DIM: usize = 8192;
    let mut image = Image::new(DIM + 2, DIM + 2, 0.0);
    let mut position = PositionIterator::new(DIM / 2 + 1, DIM / 2 + 1);
    const ODDS_COUNT: usize = DIM * DIM / 2;
    let mut is_composite_vec: BitArr!(for ODDS_COUNT) = BitArray::ZERO;
    position.next(); // 1
    let two = position.next();
    image.set(two.0, two.1, 1.0);
    for i in 1..ODDS_COUNT {
        // Iteration starts at 3
        let is_prime = !is_composite_vec[i];
        let p = position.next();
        position.next();
        if is_prime {
            let number = i * 2 + 1;
            let mut mask = i;
            while mask < ODDS_COUNT {
                is_composite_vec.set(mask, true);
                mask += number;
            }
            image.set(p.0, p.1, 1.0);
        }
    }
    image.save("prime_grid.exr")?;
    Ok(())
}
