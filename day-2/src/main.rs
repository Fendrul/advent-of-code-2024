#![feature(let_chains)]
#![feature(assert_matches)]

mod entities;

use crate::entities::report::Report;
use file_reader::file_reader::FileReader;
use std::env;
use std::error::Error;
use std::time::Instant;

const PATH: &str = env!("CARGO_MANIFEST_DIR");

#[warn(clippy::pedantic)]
fn main() -> Result<(), Box<dyn Error>> {
    let timer = Instant::now();
    let file_path = format!("{PATH}/src/input");

    let (number_safe_report, number_safe_report_with_tolerance) = solve(&file_path)?;

    println!("Number of safe reports: {number_safe_report}");
    println!("Number of safe reports with tolerance: {number_safe_report_with_tolerance}");

    println!("Finished in: {}µs", timer.elapsed().as_micros());
    Ok(())
}

fn solve(path_to_file: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let result = FileReader::new(path_to_file)?.fold((0, 0), |acc, line| {
        let numbers = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().expect("Could not parse number."))
            .collect::<Vec<_>>();

        let report = Report::new(numbers);

        let safe_value = report.get_safetyness().get_value();
        let safe_with_tolerance_value = report.get_safetyness_with_tolerance().get_value();

        (acc.0 + safe_value, acc.1 + safe_with_tolerance_value)
    });

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::entities::report::Safetyness::Safe;
    use std::assert_matches::assert_matches;

    #[test]
    fn test_main() -> Result<(), Box<dyn std::error::Error>> {
        let (result, result_with_tolerance) = solve("src/test")?;

        assert_eq!(2, result);
        assert_eq!(4, result_with_tolerance);

        let (result, result_with_tolerance) = solve("src/input")?;

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
