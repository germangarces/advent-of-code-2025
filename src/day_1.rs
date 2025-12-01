use crate::reader;
use anyhow::{Error, Result, bail};
use std::str::FromStr;

// Keep your imports (crate::reader, etc.)

#[derive(Debug)]
enum Direction {
    Clockwise,
    Counterclockwise,
}

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    steps: u32,
}

impl FromStr for Rotation {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            bail!("Invalid format");
        }
        let (d, n) = s.trim().split_at(1);
        let direction = match d {
            "L" => Direction::Counterclockwise,
            "R" => Direction::Clockwise,
            _ => bail!("Invalid direction"),
        };
        let steps = n.parse::<u32>()?;
        Ok(Self { steps, direction })
    }
}

#[derive(Clone)]
struct Dial {
    absolute_pos: i64,
    max: i64,
}

impl Dial {
    fn new(start_val: i64, max: i64) -> Self {
        Self {
            absolute_pos: start_val,
            max,
        }
    }

    fn rotate(&mut self, rotation: Rotation) -> i64 {
        let old_pos = self.absolute_pos;

        match rotation.direction {
            Direction::Clockwise => {
                self.absolute_pos += rotation.steps as i64;
                self.absolute_pos.div_euclid(self.max) - old_pos.div_euclid(self.max)
            }
            Direction::Counterclockwise => {
                self.absolute_pos -= rotation.steps as i64;
                (old_pos - 1).div_euclid(self.max) - (self.absolute_pos - 1).div_euclid(self.max)
            }
        }
    }
}

pub fn solution() -> i64 {
    let mut dial = Dial::new(50, 100);

    reader::BufReader::open("./inputs/day_1")
        .expect("Error reading file")
        .fold(0i64, |acc, line| {
            let rotation: Rotation = line.unwrap().parse().unwrap();
            let zeros_encountered = dial.rotate(rotation);

            acc + zeros_encountered
        })
}
