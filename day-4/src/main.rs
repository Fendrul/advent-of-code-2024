#![warn(clippy::pedantic)]
pub mod xmas_symbol;

use DirectionMove::{DownLeft, DownRight, UpLeft, UpRight};
use array_utils::coordinate::Coordinates;
use array_utils::{DirectionMove, TableUtils};
use file_reader::file_reader::FileReader;
use macro_utils::destruct_options;
use std::error::Error;
use std::str::Chars;
use std::time::Instant;
use xmas_symbol::XmasSymbol;

const PATH: &str = env!("CARGO_MANIFEST_DIR");

type XmasTable = Vec<Vec<XmasSymbol>>;

fn main() -> Result<(), Box<dyn Error>> {
    let timer = Instant::now();
    let file_path = format!("{PATH}/src/input");

    let (xmas_count, special_xmas_count) = solve(&file_path)?;

    println!("Xmas: {xmas_count}");
    println!("Fuck it's not xmas but x-mas: {special_xmas_count}");

    println!("Finished in: {}µs", timer.elapsed().as_micros());
    Ok(())
}

fn solve(file_path: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let (symbol_table, horizontal_xmas_count) = parse_file(file_path)?;

    let xmas_diagonal_count = symbol_table
        .iter_table_with_coordinates()
        .filter(|(symbol, ..)| **symbol == XmasSymbol::X)
        .map(|(_, coordinates)| {
            count_word_in_any_diagonal(&symbol_table, &"MAS".chars(), coordinates.to_tuple())
        })
        .sum::<usize>();

    let special_xmas_count = symbol_table
        .iter_table_with_coordinates()
        .filter(|(symbol, ..)| **symbol == XmasSymbol::A)
        .map(|(_, coordinates)| {
            let (up_left, up_right, down_left, down_right) =
                get_adjacent_symbols(&symbol_table, coordinates);

            let (up_left, up_right, down_left, down_right) =
                destruct_options!(up_left, up_right, down_left, down_right ? return 0);
            let (up_left, up_right, down_left, down_right) = (
                (up_left, UpLeft),
                (up_right, UpRight),
                (down_left, DownLeft),
                (down_right, DownRight),
            );

            let symbols_iter = [up_left, up_right, down_left, down_right].into_iter();

            if symbols_iter
                .clone()
                .filter(|symbol| *symbol.0 == XmasSymbol::S)
                .count()
                == 2
                && symbols_iter
                    .clone()
                    .filter(|symbol| *symbol.0 == XmasSymbol::M)
                    .count()
                    == 2
            {
                let var = symbols_iter
                    .filter(|symbol| *symbol.0 == XmasSymbol::M)
                    .map(|(_, direction)| direction)
                    .collect::<Vec<_>>();

                let (m1_direction, m2_direction) = (var[0], var[1]);

                if !m1_direction.is_opposed_to(m2_direction) {
                    return 1;
                }
            }

            0
        })
        .sum::<usize>();

    Ok((
        horizontal_xmas_count + xmas_diagonal_count,
        special_xmas_count,
    ))
}

fn get_adjacent_symbols(
    symbol_table: &XmasTable,
    coordinates: Coordinates,
) -> (
    Option<&XmasSymbol>,
    Option<&XmasSymbol>,
    Option<&XmasSymbol>,
    Option<&XmasSymbol>,
) {
    let up_left = symbol_table.get_from_coordinate_move(coordinates, UpLeft);
    let up_right = symbol_table.get_from_coordinate_move(coordinates, UpRight);
    let down_left = symbol_table.get_from_coordinate_move(coordinates, DownLeft);
    let down_right = symbol_table.get_from_coordinate_move(coordinates, DownRight);
    (up_left, up_right, down_left, down_right)
}

fn count_word_in_any_diagonal(
    symbol_table: &XmasTable,
    word: &Chars,
    base_coordinate: (usize, usize),
) -> usize {
    let mut count = 0;

    if match_word_in_direction(symbol_table, word.clone(), base_coordinate, DownRight) {
        count += 1;
    };

    if match_word_in_direction(symbol_table, word.clone(), base_coordinate, DownLeft) {
        count += 1;
    };

    if match_word_in_direction(symbol_table, word.clone(), base_coordinate, UpRight) {
        count += 1;
    };

    if match_word_in_direction(symbol_table, word.clone(), base_coordinate, UpLeft) {
        count += 1;
    };

    if match_word_in_direction(
        symbol_table,
        word.clone(),
        base_coordinate,
        DirectionMove::Down,
    ) {
        count += 1;
    };

    if match_word_in_direction(
        symbol_table,
        word.clone(),
        base_coordinate,
        DirectionMove::Up,
    ) {
        count += 1;
    };

    count
}

fn match_word_in_direction(
    symbol_table: &XmasTable,
    mut word: Chars,
    base_coordinate: (usize, usize),
    direction: DirectionMove,
) -> bool {
    let (move_x, move_y) = direction.get_direction();
    let symbol_to_match = match word.next() {
        Some(char) => XmasSymbol::from(char),
        None => return true, // If it get to this point, all chars of the word have been checked
    };

    let x_destination = base_coordinate.0.checked_add_signed(move_x);
    let y_destination = base_coordinate.1.checked_add_signed(move_y);

    let (x_destination, y_destination) =
        destruct_options!(x_destination, y_destination ? return false);

    if symbol_table.get_from_coordinates(x_destination, y_destination) != Some(&symbol_to_match) {
        return false;
    }

    match_word_in_direction(
        symbol_table,
        word,
        (x_destination, y_destination),
        direction,
    )
}

fn parse_file(file_path: &str) -> Result<(XmasTable, usize), Box<dyn Error>> {
    let result = FileReader::new(file_path)?.fold((Vec::new(), 0), |mut acc, line| {
        let symbol_table = &mut acc.0;
        let xmas_count = &mut acc.1;

        *xmas_count += line.matches("XMAS").count();
        *xmas_count += line.matches("SAMX").count();

        let symbols_vec = line.trim().chars().map(XmasSymbol::from).collect();

        symbol_table.push(symbols_vec);

        acc
    });

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solver() -> Result<(), Box<dyn Error>> {
        let path_file = format!("{PATH}/src/test");

        let (result, result_two) = solve(&path_file)?;
        assert_eq!(18, result);
        assert_eq!(9, result_two);

        Ok(())
    }

    #[test]
    fn test_puzzle_result() -> Result<(), Box<dyn Error>> {
        let path_file = format!("{PATH}/src/input");
        let (result, result_two) = solve(&path_file)?;
        assert_eq!(2530, result);
        assert_eq!(1921, result_two);

        Ok(())
    }
}
