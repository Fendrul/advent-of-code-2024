#![warn(clippy::pedantic)]

mod page_rule;

use crate::page_rule::PageRule;
use file_reader::file_reader::FileReader;
use std::collections::HashSet;
use std::error::Error;
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

    let page_rules_set =
        page_rules
            .iter()
            .fold(HashSet::new(), |mut set, (left_num, right_num)| {
                let left_page = get_or_create_page_rule(&mut set, *left_num);
                let right_page = get_or_create_page_rule(&mut set, *right_num);

                left_page.add_page_after(right_page.clone());

                set
            });

    let mut sum_valid_updates = 0;
    let mut invalid_updates = Vec::new();

    for numbers in updates {
        let mut pages_to_be_present = HashSet::new();

        let mut is_update_valid = true;
        for current_number in numbers.iter().rev() {
            let page = page_rules_set
                .get(current_number)
                .unwrap_or_else(|| panic!("Page {} should be in the set", current_number));
            pages_to_be_present.extend(page.get_pages_number_after());

            if pages_to_be_present.contains(current_number) {
                is_update_valid = false;
                break;
            }
        }

        if is_update_valid {
            sum_valid_updates += get_middle_number_from_vec(numbers);
        } else {
            invalid_updates.push(
                numbers
                    .iter()
                    .map(|number| {
                        page_rules_set
                            .get(number)
                            .expect("Page for invalid update should be in the set")
                            .clone()
                    })
                    .collect::<Vec<_>>(),
            );
        }
    }

    let sum_reordered_invalid_updates = invalid_updates
        .iter_mut()
        .map(|page_rules| {
            let update_len = page_rules.len();
            let mut corrected_update = Vec::new();

            while corrected_update.len() != update_len {
                let numbers_to_filter = page_rules.iter().fold(HashSet::new(), |mut set, page| {
                    set.extend(page.get_pages_number_after());
                    set
                });

                let number_to_add = page_rules
                    .iter()
                    .map(PageRule::get_number)
                    .find(|number| !numbers_to_filter.contains(number))
                    .unwrap_or_else(|| panic!("Couldn't find number for update: {:?}", page_rules));

                corrected_update.push(number_to_add);
                page_rules.retain(|page| page.get_number() != number_to_add);
            }

            get_middle_number_from_vec(corrected_update)
        })
        .sum::<usize>();

    Ok((sum_valid_updates, sum_reordered_invalid_updates))
}

fn get_middle_number_from_vec(numbers: Vec<usize>) -> usize {
    *numbers
        .get(numbers.len() / 2)
        .expect("Middle element should exist")
}

fn get_or_create_page_rule(page_rules_set: &mut HashSet<PageRule>, page_number: usize) -> PageRule {
    if let Some(page) = page_rules_set.get(&page_number) {
        page.clone()
    } else {
        let page = PageRule::new(page_number);
        page_rules_set.insert(page.clone());
        page
    }
}

fn parse_rule(str: &str) -> (usize, usize) {
    let mut split = str.split("|");

    let error_find_str = &format!("Couldn't extract number in line: {str}");
    let error_convert_str = &format!("Couldn't convert number in line: {str}");

    let left_number = split
        .next()
        .expect(error_find_str)
        .trim()
        .parse::<usize>()
        .expect(error_convert_str);
    let right_number = split
        .next()
        .expect(error_find_str)
        .trim()
        .parse::<usize>()
        .expect(error_convert_str);

    (left_number, right_number)
}

fn parse_update(str: &str) -> Vec<usize> {
    let error_convert_str = &format!("Couldn't convert number in line: {str}");

    str.split(",")
        .map(|number| number.trim().parse::<usize>().expect(error_convert_str))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;

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

    #[test]
    fn playground() {
        #[derive(Clone, Debug)]
        struct Test {
            number: usize,
        }

        impl Test {
            fn set_number(&mut self, number: usize) {
                self.number = number;
            }
        }

        let refcell = RefCell::new(Test { number: 1 });

        let clone = refcell.clone();

        refcell.borrow_mut().set_number(2);

        println!("{:#?}", clone);
    }

    #[test]
    fn test_get_existing_page_rule() {
        let mut page_rules_set = HashSet::new();
        let initial_page = PageRule::new(1);
        page_rules_set.insert(initial_page);

        let result = get_or_create_page_rule(&mut page_rules_set, 1);
        assert_eq!(result.get_number(), 1);
        assert!(result.get_pages_number_after().is_empty());
    }

    #[test]
    fn test_create_new_page_rule() {
        let mut page_rules_set = HashSet::new();
        let result = get_or_create_page_rule(&mut page_rules_set, 2);

        assert_eq!(result.get_number(), 2);
        assert!(result.get_pages_number_after().is_empty());
        assert!(page_rules_set.contains(&result));
    }

    #[test]
    fn test_multiple_operations() {
        let mut page_rules_set = HashSet::new();

        // Create first page rule
        let page1 = get_or_create_page_rule(&mut page_rules_set, 1);
        assert_eq!(page_rules_set.len(), 1);

        // Get existing page rule
        let page1_again = get_or_create_page_rule(&mut page_rules_set, 1);
        assert_eq!(page_rules_set.len(), 1);
        assert_eq!(page1.get_number(), page1_again.get_number());

        // Create second page rule
        let page2 = get_or_create_page_rule(&mut page_rules_set, 2);
        assert_eq!(page_rules_set.len(), 2);
        assert_ne!(page1.get_number(), page2.get_number());
    }

    #[test]
    fn test_retrieve_page_rule_modifications() {
        let mut page_rules_set = HashSet::new();
        let page1 = get_or_create_page_rule(&mut page_rules_set, 1);
        let page2 = get_or_create_page_rule(&mut page_rules_set, 2);

        page1.add_page_after(page2.clone());

        let page1 = get_or_create_page_rule(&mut page_rules_set, 1);
        let pages_after = page1.get_pages_number_after();
        assert_eq!(pages_after.len(), 1);
    }
}
