use Safetyness::{Safe, Unsafe};
use std::cmp::PartialEq;

#[derive(Debug)]
pub struct Report {
    numbers: Vec<i32>,
    safetyness: Safetyness,
}

impl Report {
    pub fn new(numbers: Vec<i32>) -> Report {
        Report {
            safetyness: give_safetyness(&numbers),
            numbers,
        }
    }

    pub fn get_safetyness(&self) -> &Safetyness {
        &self.safetyness
    }

    pub fn get_safetyness_with_tolerance(&self) -> Safetyness {
        match self.get_safetyness() {
            Safe => Safe,
            Unsafe => {
                for i in 0..self.numbers.len() {
                    let mut current_vec = self.numbers.clone();
                    current_vec.remove(i);

                    if Safe == give_safetyness(&current_vec) {
                        return Safe;
                    }
                }

                Unsafe
            }
        }
    }
}

fn give_safetyness(slice: &[i32]) -> Safetyness {
    let interval = 1;

    let mut global_direction: Option<Direction> = None;

    for (current, next) in slice.iter().zip(slice.iter().skip(interval)) {
        global_direction = if global_direction.is_some() {
            global_direction
        } else {
            Some(Direction::from(*current, *next))
        };

        if global_direction != Some(Direction::from(*current, *next)) {
            return Unsafe;
        }

        let diff = (current - next).abs();
        if diff > 3 || diff == 0 {
            return Unsafe;
        }
    }

    Safe
}

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    Increasing,
    Decreasing,
}

impl Direction {
    fn from(x: i32, y: i32) -> Direction {
        if x < y {
            Direction::Increasing
        } else {
            Direction::Decreasing
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Safetyness {
    Safe,
    Unsafe,
}
