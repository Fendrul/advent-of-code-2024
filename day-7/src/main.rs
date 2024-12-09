#![warn(clippy::pedantic)]
#![allow(dead_code)]

mod calibration;

use crate::calibration::{Calibration, Expression};
use file_reader::file_reader::FileReader;
use num_bigint::BigUint;
use std::error::Error;
use std::time::Instant;

const PATH: &str = env!("CARGO_MANIFEST_DIR");

fn main() -> Result<(), Box<dyn Error>> {
    let timer = Instant::now();
    let file_path = format!("{PATH}/src/input");

    let (total_calibration, total_corrected_calibration) = solve(&file_path)?;

    println!("part 1: {total_calibration}");
    println!("part 2: {total_corrected_calibration}");

    println!("Finished in: {}ms", timer.elapsed().as_millis());
    Ok(())
}

fn solve(file_path: &str) -> Result<(usize, BigUint), Box<dyn Error>> {
    let calibrations = FileReader::new(file_path)?
        .map(|line| parse_line(line))
        .collect::<Vec<_>>();

    let number_corresponding_expr = calibrations
        .iter()
        .filter(|calibration| calibration.has_matching_expression_first_part())
        .map(Calibration::get_result)
        .sum::<usize>();

    let second_part = calibrations
        .iter()
        .filter(|calibration| calibration.has_matching_expression_second_part())
        .map(Calibration::get_result)
        .sum::<BigUint>();

    Ok((number_corresponding_expr, second_part))
}

fn parse_line(line: String) -> Calibration {
    let mut split = line.split(':');

    let value = split
        .next()
        .expect("Couldn't extract the result value")
        .parse::<usize>()
        .expect("Couldn't parse the result value");

    let split = split
        .next()
        .expect("Couldn't extract the expression")
        .split_whitespace();

    let expr = parse_expr(&split.rev().collect::<Vec<_>>());

    Calibration::new(value, expr)
}

fn parse_expr(expression: &[&str]) -> Expression {
    let number = expression[0]
        .parse::<usize>()
        .expect("Couldn't parse the number");

    if expression.len() > 1 {
        let expr = parse_expr(&expression[1..]);

        return Expression::Expr(number, Box::new(expr));
    }

    Expression::Litteral(number)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn Error>> {
        let file_path = format!("{PATH}/src/test");
        let (total_calibration, right_calibration) = solve(&file_path)?;

        assert_eq!(total_calibration, 3749, "Couldn't solve the first part");
        assert_eq!(
            right_calibration,
            BigUint::from(11387usize),
            "Couldn't solve the second part"
        );

        Ok(())
    }
}
