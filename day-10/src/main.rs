#![feature(let_chains)]
#![warn(clippy::pedantic)]

mod entities;

use crate::entities::MapCell;
use entities::{Ghost, Map};
use file_reader::file_reader::FileReader;
use itertools::Itertools;
use std::error::Error;
use std::time::Instant;

const PATH: &str = env!("CARGO_MANIFEST_DIR");

fn main() -> Result<(), Box<dyn Error>> {
    let timer = Instant::now();
    let file_path = format!("{PATH}/input");

    let (trailheads_count, rating_trailheads_count) = solve(&file_path)?;

    println!("Trailheads: {trailheads_count}");
    println!("Trailheads with rating: {rating_trailheads_count}");

    println!("Finished in: {}µs", timer.elapsed().as_micros());
    Ok(())
}

fn solve(file_path: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let map_cells = FileReader::new(file_path)?
        .enumerate()
        .map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    let num_value = c
                        .to_digit(10)
                        .unwrap_or_else(|| panic!("Couldn't convert the char into digit: {c}"));
                    MapCell::new(num_value as usize, (x, y))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let map = Map::new(map_cells);

    let trailheads_count = map
        .give_starting_ghosts()
        .into_iter()
        .flat_map(|ghost| ghost.explore().unique())
        .count();

    let rating_trailheads_count = map
        .give_starting_ghosts()
        .into_iter()
        .flat_map(Ghost::explore)
        .count();

    Ok((trailheads_count, rating_trailheads_count))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn Error>> {
        let file_path = format!("{PATH}/test");
        let (trailheads_count, result2) = solve(&file_path)?;

        assert_eq!(trailheads_count, 36, "Test for first part failed");
        assert_eq!(result2, 81, "Test for second part failed");

        Ok(())
    }

    #[test]
    fn test_solve2() -> Result<(), Box<dyn Error>> {
        let file_path = format!("{PATH}/test2");
        let (trailheads_count, _) = solve(&file_path)?;

        assert_eq!(trailheads_count, 2, "Test case 2 for first part failed");

        Ok(())
    }

    #[test]
    fn test_solution() -> Result<(), Box<dyn Error>> {
        let file_path = format!("{PATH}/input");
        let (trailheads_count, result2) = solve(&file_path)?;

        assert_eq!(trailheads_count, 607, "Solution for first part failed");
        assert_eq!(result2, 1384, "Solution for second part failed");

        Ok(())
    }
}
