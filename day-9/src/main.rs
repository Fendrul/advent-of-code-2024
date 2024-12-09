#![feature(iter_collect_into)]

mod file_block;

use crate::file_block::{FileBlock, FileBlockType};
use file_reader::file_reader::FileReader;
use std::error::Error;
use std::time::Instant;

const PATH: &str = env!("CARGO_MANIFEST_DIR");

fn main() -> Result<(), Box<dyn Error>> {
    let timer = Instant::now();
    let file_path = format!("{PATH}/input");

    let (fragmented_sum, unfragmented_sum) = solve(&file_path)?;

    println!("File system checksum: {fragmented_sum}");
    println!("part 2: {unfragmented_sum}");

    println!("Finished in: {}µs", timer.elapsed().as_micros());
    Ok(())
}

fn solve(file_path: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let parse = parse_file(file_path)?;

    let fragmented_sum = get_sum_from_fragmented(&parse);

    let file_block_rev = parse
        .iter()
        .filter(|file_block| matches!(file_block.get_type(), FileBlockType::Number(_)))
        .map(|file_block| {
            if let FileBlockType::Number(digit) = file_block.get_type() {
                (file_block.get_size(), digit)
            } else {
                panic!("Should not find a void block after the filter")
            }
        })
        .rev()
        .collect::<Vec<_>>();

    let mut index = 0;
    let mut sum = 0;

    'outer: for file_block in parse {
        if let FileBlockType::Number(number) = file_block.get_type() {
            let number = i32::try_from(number)
                .expect(format!("Invalid number to convert into the i32: {number}").as_str());

            sum += number * (index + file_block.get_size()) / 2;
            index += file_block.get_size();
        } else {
            let length_to_find = file_block.get_size();

            if let Some((length, number)) = file_block_rev
                .iter()
                .find(|(length, _)| length >= &length_to_find)
            {
                let number = i32::try_from(*number)
                    .expect(format!("Invalid number to convert into the i32: {number}").as_str());
                sum += number * (index + length_to_find) / 2;
                index += length_to_find;
                
                
            }
        }
    }

    Ok((fragmented_sum, 0))
}

fn parse_file(file_path: &str) -> Result<Vec<FileBlock>, Box<dyn Error>> {
    let parse = FileReader::new(file_path)?
        .next()
        .expect("No line in file")
        .trim()
        .chars()
        .enumerate()
        .map(|(index, char)| {
            let digit_count = match char {
                char if char.is_numeric() => char.to_digit(10).expect("Invalid digit"),
                _ => panic!("Invalid character for parsing"),
            };

            let file_block_type = if index % 2 == 0 {
                FileBlockType::Number(index / 2)
            } else {
                FileBlockType::Void
            };

            FileBlock::new(digit_count as i32, file_block_type)
        })
        .collect::<Vec<_>>();
    Ok(parse)
}

fn get_sum_from_fragmented(parse: &[FileBlock]) -> usize {
    let decomposed_file_block = parse
        .iter()
        .flat_map(|file_block| (0..file_block.get_size()).map(|_| file_block.get_type()))
        .collect::<Vec<_>>();

    let mut file_block_rev_iter = decomposed_file_block
        .iter()
        .enumerate()
        .filter(|(_, file_block)| matches!(file_block, FileBlockType::Number(_)))
        .map(|(index, file_block)| {
            if let FileBlockType::Number(number) = file_block {
                (index, number)
            } else {
                panic!("Should not find a void block after the filter")
            }
        })
        .rev();

    let mut sum = 0;
    let mut last_elem_index = usize::MAX;
    'outer: for (index, file_block) in decomposed_file_block.iter().enumerate() {
        if index >= last_elem_index {
            break 'outer;
        }

        if let FileBlockType::Number(number) = file_block {
            sum += number * index;
        } else {
            let (last_elem_index_temp, last_elem_value) = file_block_rev_iter
                .next()
                .expect("Should still find elements in the reverse iterator");
            last_elem_index = last_elem_index_temp;

            sum += last_elem_value * index;
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn Error>> {
        let file_path = format!("{PATH}/test");
        let (file_system_checksum, result2) = solve(&file_path)?;
        assert_eq!(file_system_checksum, 1928);
        assert_eq!(result2, 0);

        Ok(())
    }

    #[test]
    fn test_solution() -> Result<(), Box<dyn Error>> {
        let file_path = format!("{PATH}/input");
        let (file_system_checksum, result2) = solve(&file_path)?;

        assert_eq!(file_system_checksum, 6461289671426);
        assert_eq!(result2, 0);

        Ok(())
    }
}
