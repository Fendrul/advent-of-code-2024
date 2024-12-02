use Ordering::{Equal, Greater, Less};
use Safetyness::{Safe, Unsafe};
use std::cmp::{Ordering, PartialEq};
use std::iter::{Skip, Zip};
use std::slice::Iter;
use Direction::{Decreasing, Increasing, Flat};

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
    let mut global_direction: Option<Direction> = None;

    for (current, next) in slice.pair_wise(1) {
        global_direction = if global_direction.is_some() {
            global_direction
        } else {
            Some(Direction::from(*current, *next))
        };

        if global_direction != Some(Direction::from(*current, *next)) || matches!(global_direction, Some(Flat)){
            return Unsafe;
        }

        if (current - next).abs() > 3 {
            return Unsafe;
        }
    }

    Safe
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Increasing,
    Decreasing,
    Flat
}

impl Direction {
    fn from(x: i32, y: i32) -> Direction {
        match x.cmp(&y) {
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
    pub fn get_value(&self) -> i32 {
        match self {
            Safe => 1,
            Unsafe => 0,
        }
    }
}
pub trait VecUtils<T> {
    fn pair_wise(&self, interval: usize) -> Zip<Iter<'_, T>, Skip<Iter<'_, T>>>;
}

impl<T> VecUtils<T> for &[T] {
    fn pair_wise(&self, interval: usize) -> Zip<Iter<'_, T>, Skip<Iter<'_, T>>> {
        self.iter().zip(self.iter().skip(interval))
    }
}