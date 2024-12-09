#![warn(clippy::pedantic)]

mod antenna;

use crate::antenna::AntennaCoordinates;
use file_reader::file_reader::FileReader;
use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

const PATH: &str = env!("CARGO_MANIFEST_DIR");

fn main() -> Result<(), Box<dyn Error>> {
    let timer = Instant::now();
    let file_path = format!("{PATH}/input");

    let (total_calibration, total_corrected_calibration) = solve(&file_path)?;

    println!("part 1: {total_calibration}");
    println!("part 2: {total_corrected_calibration}");

    println!("Finished in: {}µs", timer.elapsed().as_micros());
    Ok(())
}

fn solve(file_path: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let antennas_coordinates =
        FileReader::new(file_path)?
            .enumerate()
            .fold(HashMap::new(), |mut hasm_map, (y, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .filter(|(_, c)| *c != '.')
                    .for_each(|(x, char)| {
                        hasm_map
                            .entry(char)
                            .or_insert_with(AntennaCoordinates::new)
                            .add_coordinates((x as isize, y as isize));
                    });

                hasm_map
            });

    let max_y = FileReader::new(file_path)?.count() as isize;
    let max_x = FileReader::new(file_path)?.next().unwrap().trim().len() as isize;

    let count_coordinates = antennas_coordinates
        .iter()
        .flat_map(|(_, coordinates)| coordinates.get_extrapolated_coordinates())
        .filter(|(x, y)| (0..max_x).contains(x) && (0..max_y).contains(y))
        .unique()
        .count();

    let new_count_coordinates = antennas_coordinates
        .iter()
        .flat_map(|(_, coordinates)| {
            coordinates.get_extrapolated_coordinates_with_repetition(max_x, max_y)
        })
        .unique()
        .count();

    Ok((count_coordinates, new_count_coordinates))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn Error>> {
        let file_path = format!("{PATH}/test");
        let (total_calibration, total_corrected_calibration) = solve(&file_path)?;
        assert_eq!(total_calibration, 14, "Couldn't solve part 1");
        assert_eq!(total_corrected_calibration, 34, "Couldn't solve part 2");

        Ok(())
    }

    #[test]
    fn second_test() -> Result<(), Box<dyn Error>> {
        let file_path = format!("{PATH}/test2");
        let (_, total_corrected_calibration) = solve(&file_path)?;
        assert_eq!(total_corrected_calibration, 9, "Couldn't solve part 2");

        Ok(())
    }
}
