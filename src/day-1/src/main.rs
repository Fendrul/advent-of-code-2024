use file_reader::file_reader::FileReader;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timer = std::time::Instant::now();

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    FileReader::new("src/day-1/src/input")?.for_each(|line| {
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

        left_list.push(left_number);
        right_list.push(right_number);
    });

    left_list.sort();
    right_list.sort();

    let frequency_map = right_list.iter().fold(HashMap::new(), |mut map, number| {
        let x = map.entry(number).or_insert(0);

        *x += 1;

        map
    });

    let mut total_sum = 0;
    let mut similarity_score = 0;

    for (left_number, right_number) in left_list.iter().zip(right_list.iter()) {
        let value = (left_number - right_number).abs();

        total_sum += value;

        similarity_score += left_number * *frequency_map.get(left_number).get_or_insert(&0);
    }

    println!("The total sum is: {}", total_sum);
    println!("The similarity score is: {}", similarity_score);

    println!("Finished in: {}µm", timer.elapsed().as_micros());
    Ok(())
}
