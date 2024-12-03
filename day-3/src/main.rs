use crate::entities::parser::parse_stdin;
use file_reader::file_reader::FileReader;
use std::error::Error;
use std::time::Instant;

mod entities;

const PATH: &str = env!("CARGO_MANIFEST_DIR");
#[warn(clippy::pedantic)]
fn main() -> Result<(), Box<dyn Error>> {
    let timer = Instant::now();
    let file_path = format!("{PATH}/src/input");

    let (result, result_by_instruction) = solve(&file_path)?;

    println!("Result: {result}");
    println!("Result by instruction: {result_by_instruction}");

    println!("Finished in: {}µs", timer.elapsed().as_micros());
    Ok(())
}

fn solve(file_path: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let file = FileReader::read_file(file_path)?;

    let mul_expressions = parse_stdin(&file)?;

    let result = mul_expressions.iter().fold((0, 0), |acc, mul_expression| {
        let result = mul_expression.evaluate();
        let result_by_instruction = mul_expression.evaluate_by_instruction();

        (acc.0 + result, acc.1 + result_by_instruction)
    });
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() -> Result<(), Box<dyn Error>> {
        let path_file = format!("{PATH}/src/test");
        let (result, result_by_instruction) = solve(&path_file)?;
        assert_eq!(161, result);
        assert_eq!(48, result_by_instruction);

        let path_file = format!("{PATH}/src/input");
        let (result, result_by_instruction) = solve(&path_file)?;
        assert_eq!(165225049, result);
        assert_eq!(108830766, result_by_instruction);

        Ok(())
    }
}
