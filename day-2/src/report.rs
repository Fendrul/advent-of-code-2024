use Direction::{Decreasing, Flat, Increasing};
use Ordering::{Equal, Greater, Less};
use Safetyness::{Safe, Unsafe};
use array_utils::ArrayUtils;
use std::cmp::{Ordering, PartialEq};

#[derive(Debug)]
pub struct Report {
    numbers: Vec<usize>,
    safetyness: Safetyness,
}

impl Report {
    pub fn new(numbers: Vec<usize>) -> Report {
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

fn give_safetyness(slice: &[usize]) -> Safetyness {
    let mut global_direction: Option<Direction> = None;

    for (current, next) in slice.pairwise(1) {
        global_direction = global_direction.or_else(|| Some(Direction::from(current, next)));

        if global_direction != Some(Direction::from(current, next))
            || matches!(global_direction, Some(Flat))
        {
            return Unsafe;
        }

        if current.abs_diff(*next) > 3 {
            return Unsafe;
        }
    }

    Safe
}

#[derive(Debug, PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
    Flat,
}

impl Direction {
    fn from(x: &usize, y: &usize) -> Direction {
        match x.cmp(y) {
            Less => Increasing,
            Greater => Decreasing,
            Equal => Flat,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Safetyness {
    Safe,
    Unsafe,
}

impl Safetyness {
    pub fn get_value(&self) -> usize {
        match self {
            Safe => 1,
            Unsafe => 0,
        }
    }
}
