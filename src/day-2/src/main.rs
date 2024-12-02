#![feature(let_chains)]
#![feature(assert_matches)]

mod entities;

use crate::entities::report::{Report};
use file_reader::file_reader::FileReader;
use std::error::Error;
use std::time::Instant;

#[warn(clippy::pedantic)]
fn main() -> Result<(), Box<dyn Error>> {
    let timer = Instant::now();
    let path_to_file = "src/day-2/src/input";

    let (number_safe_report, number_safe_report_with_tolerance) = solve(path_to_file)?;

    println!("Number of safe reports: {number_safe_report}");
    println!("Number of safe reports with tolerance: {number_safe_report_with_tolerance}");

    println!("Finished in: {}µs", timer.elapsed().as_micros());
    Ok(())
}

fn solve(path_to_file: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let (safe_count, safe_with_tolerance_count) = FileReader::new(path_to_file)?
        .map(|line| {
            let numbers = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let report = Report::new(numbers);
            
            let is_safe = report.get_safetyness().get_value();
            let is_safe_with_tolerance = report.get_safetyness_with_tolerance().get_value();

            (is_safe, is_safe_with_tolerance)
        })
        .fold((0, 0), |mut acc, (safe_count, safe_with_tolerance_count)| {
            acc.0 += safe_count;
            acc.1 += safe_with_tolerance_count;
            
            acc
        });

    Ok((safe_count, safe_with_tolerance_count))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::entities::report::Safetyness::Safe;
    use std::assert_matches::assert_matches;

    #[test]
    fn test_main() -> Result<(), Box<dyn std::error::Error>> {
        let (result, result_with_tolerance) = solve("src/day-2/src/test")?;

        assert_eq!(2, result);
        assert_eq!(4, result_with_tolerance);

        let (result, result_with_tolerance) = solve("src/day-2/src/input")?;

        assert_eq!(390, result);
        assert_eq!(439, result_with_tolerance);

        Ok(())
    }

    #[test]
    fn test_safetyness() {
        let report = Report::new(vec![8, 6, 4, 4, 1]);

        let safetyness = report.get_safetyness_with_tolerance();

        assert_matches!(safetyness, Safe);
    }
}
