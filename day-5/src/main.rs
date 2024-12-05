#![warn(clippy::pedantic)]

mod page_rule;

use crate::page_rule::PageRule;
use file_reader::file_reader::FileReader;
use std::collections::HashSet;
use std::error::Error;
use std::str::Split;
use std::time::Instant;

const PATH: &str = env!("CARGO_MANIFEST_DIR");

fn main() -> Result<(), Box<dyn Error>> {
    let timer = Instant::now();
    let file_path = format!("{PATH}/src/input");

    let (sum_valid_updates, sum_reordered_invalid_updates) = solve(&file_path)?;

    println!("Sum for valid updates: {sum_valid_updates}");
    println!("Sum for reordered invalid updates: {sum_reordered_invalid_updates}");

    println!("Finished in: {}ms", timer.elapsed().as_millis());
    Ok(())
}

fn solve(file_path: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let (page_rules, updates): (Vec<_>, Vec<_>) = FileReader::new(file_path)?
        .filter(|line| !line.trim().is_empty())
        .partition(|line| line.contains('|'));

    let page_rules: Vec<_> = page_rules.iter().map(|line| parse_rule(line)).collect();
    let updates: Vec<_> = updates.iter().map(|line| parse_update(line)).collect();

    let page_rules_set = page_rules.iter().fold(
        HashSet::new(),
        |mut set: HashSet<PageRule>, (left_num, right_num)| {
            append_number_to_page(&mut set, *left_num, *right_num);

            set
        },
    );

    let mut sum_valid_updates = 0;
    let mut invalid_updates_page_rules = Vec::new();

    for update in updates {
        let mut pages_not_to_be_present: HashSet<usize> = HashSet::new();

        let mut is_update_valid = true;

        update.iter().rev().for_each(|page_number| {
            if let Some(page_number) = page_rules_set.get(page_number) {
                pages_not_to_be_present.extend(page_number.get_ref_pages_number_after().iter());
            }

            if pages_not_to_be_present.contains(page_number) {
                is_update_valid = false;
            }
        });

        if is_update_valid {
            sum_valid_updates += get_middle_number_from_vec(update);
        } else {
            invalid_updates_page_rules.push(
                update
                    .iter()
                    .filter_map(|number| page_rules_set.get(number))
                    .collect::<Vec<_>>(),
            );
        }
    }

    let sum_reordered_invalid_updates = invalid_updates_page_rules
        .iter_mut()
        .map(|update| {
            let update_len = update.len();
            let mut corrected_update = Vec::new();

            while corrected_update.len() != update_len {
                let numbers_to_filter =
                    update
                        .iter()
                        .fold(HashSet::new(), |mut set: HashSet<usize>, page| {
                            set.extend(page.get_ref_pages_number_after().iter());
                            set
                        });

                let number_to_add = update
                    .iter()
                    .map(|page_rule| page_rule.get_number())
                    .find(|number| !numbers_to_filter.contains(number))
                    .unwrap_or_else(|| panic!("Couldn't find number for update: {:?}", update));

                corrected_update.push(number_to_add);
                update.retain(|page| page.get_number() != number_to_add);
            }

            get_middle_number_from_vec(corrected_update)
        })
        .sum::<usize>();

    Ok((sum_valid_updates, sum_reordered_invalid_updates))
}

fn append_number_to_page(set: &mut HashSet<PageRule>, left_num: usize, right_num: usize) {
    if let Some(left_page) = set.get(&left_num) {
        left_page.add_page_number_after(&right_num);
    } else {
        let new_page = PageRule::new(left_num);
        new_page.add_page_number_after(&right_num);
        set.insert(new_page);
    }
}

fn get_middle_number_from_vec(numbers: Vec<usize>) -> usize {
    *numbers
        .get(numbers.len() / 2)
        .expect("Middle element should exist")
}

fn parse_rule(str: &str) -> (usize, usize) {
    let mut split = str.split("|");

    let left_number = parse_next_split_value(&mut split);
    let right_number = parse_next_split_value(&mut split);

    (left_number, right_number)
}

fn parse_next_split_value(split: &mut Split<&str>) -> usize {
    split
        .next()
        .expect("Couldn't extract number.")
        .trim()
        .parse::<usize>()
        .expect("Couldn't convert number.")
}

fn parse_update(str: &str) -> Vec<usize> {
    let error_convert_str = &format!("Couldn't convert number in line: {str}");

    str.split(',')
        .map(|number| number.trim().parse::<usize>().expect(error_convert_str))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solver() -> Result<(), Box<dyn Error>> {
        let path_file = format!("{PATH}/src/test");

        let (result, result_two) = solve(&path_file)?;
        assert_eq!(143, result);
        assert_eq!(123, result_two);

        Ok(())
    }

    #[test]
    fn test_solution() -> Result<(), Box<dyn Error>> {
        let path_file = format!("{PATH}/src/input");
        let (result, result_two) = solve(&path_file)?;
        assert_eq!(4924, result);
        assert_eq!(6085, result_two);

        Ok(())
    }
}
