use file_reader::file_reader::FileReader;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

type ParsingData = (Vec<i32>, Vec<i32>, HashMap<i32, i32>);

fn main() -> Result<(), Box<dyn Error>> {
    let timer = Instant::now();

    let (total_sum, similarity_score) = solve_puzzle("src/day-1/src/input")?;

    println!("The total sum is: {total_sum}");
    println!("The similarity score is: {similarity_score}");

    println!("Finished in: {}µs", timer.elapsed().as_micros());
    Ok(())
}

fn solve_puzzle(path_to_file: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let (mut left_vec, mut right_vec, frequency_map) = parse(path_to_file)?;

    left_vec.sort_unstable();
    right_vec.sort_unstable();

    let (total_sum, similarity_score) = calculate_result(&left_vec, &right_vec, &frequency_map);

    Ok((total_sum, similarity_score))
}

fn calculate_result(
    left_list: &[i32],
    right_list: &[i32],
    frequency_map: &HashMap<i32, i32>,
) -> (i32, i32) {
    let mut total_sum = 0;
    let mut similarity_score = 0;

    for (left_number, right_number) in left_list.iter().zip(right_list.iter()) {
        let distance = (left_number - right_number).abs();

        total_sum += distance;
        similarity_score += left_number * *frequency_map.get(left_number).get_or_insert(&0);
    }

    (total_sum, similarity_score)
}

fn parse(path_to_file: &str) -> Result<ParsingData, Box<dyn Error>> {
    let mut left_vec = Vec::new();
    let mut right_vec = Vec::new();
    let mut frequency_map = HashMap::new();

    FileReader::new(path_to_file)?.for_each(|line| {
        let mut split = line.split_whitespace();

        let left_number = split
            .next()
            .expect("Failed to get left number.")
            .parse::<i32>()
            .expect("Failed to parse left number.");

        let right_number = split
            .next()
            .expect("Failed to get right number.")
            .parse::<i32>()
            .expect("Failed to parse right number.");

        left_vec.push(left_number);
        right_vec.push(right_number);

        let frequency_count = frequency_map.entry(left_number).or_insert(0);
        *frequency_count += 1;
    });

    Ok((left_vec, right_vec, frequency_map))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() -> Result<(), Box<dyn Error>> {
        let (total_sum, similarity_score) = solve_puzzle("src/day-1/src/test")?;

        assert_eq!(total_sum, 11);
        assert_eq!(similarity_score, 31);

        Ok(())
    }
}
