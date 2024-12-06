#![feature(let_chains)]
#![warn(clippy::pedantic)]

use crate::map_element::GuardDirection::{Left, Right, Up};
use crate::map_element::{GuardDirection, MapType};
use GuardDirection::Down;
use array_utils::{DirectionMove, TableUtils, move_coordinates};
use file_reader::file_reader::FileReader;
use map_element::MapElement;
use std::error::Error;
use std::time::Instant;
use MapType::{Guard, Junk, Void};

mod map_element;

const PATH: &str = env!("CARGO_MANIFEST_DIR");

fn main() -> Result<(), Box<dyn Error>> {
    let timer = Instant::now();
    let file_path = format!("{PATH}/src/input");

    let (tiles_visited_count, blocking_count) = solve(&file_path)?;

    println!("Visited tiles: {tiles_visited_count}");
    println!("Number of blocking positions: {blocking_count}");

    println!("Finished in: {}µs", timer.elapsed().as_micros());
    Ok(())
}

fn solve(path_to_file: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let mut board = FileReader::new(path_to_file)?
        .enumerate()
        .map(|(y_index, line)| {
            let line = line.trim();
            let mut map_elements = Vec::new();

            for (x_index, char) in line.chars().enumerate() {
                let new_map_element = MapElement::new(char, x_index, y_index);
                map_elements.push(new_map_element);
            }

            map_elements
        })
        .collect::<Vec<_>>();

    let guard = board
        .iter()
        .flat_map(|row| row.iter())
        .find(|map_element| matches!(map_element.get_map_type(), Guard(..)))
        .expect("No guard found");
    let mut guard_direction = if let Guard(guard_direction, _) = guard.get_map_type() {
        *guard_direction
    } else {
        panic!("We are assured that element found is a guard");
    };
    let (mut guard_x, mut guard_y) = guard.get_coordinates();

    while let Some(next_tile) = board
        .get_mut_from_coordinates_move((guard_x, guard_y), DirectionMove::from(guard_direction))
    {
        match next_tile.get_mut_map_type() {
            Junk => guard_direction = guard_direction.turn_right(),
            &mut (Void(ref mut directions) | Guard(_, ref mut directions)) => {
                directions.push(guard_direction);

                (guard_x, guard_y) = move_tile(guard_direction, guard_x, guard_y);
            }
        }
    }

    let visited_tiles_count = board
        .iter()
        .flat_map(|row| row.iter())
        .filter(|map_element| {
            if let Void(visited_state) = map_element.get_map_type() {
                !visited_state.is_empty()
            } else {
                false
            }
        })
        .count()
        + 1;

    let blocking_count = board
        .iter()
        .flat_map(|row| row.iter())
        .filter(|map_element| matches!(map_element.get_map_type(), MapType::Junk))
        .map(|junk_element| {
            let mut blocking_tiles_found = 0;

            'outer: for direction in [Down, Up, Left, Right] {
                let (mut x, mut y) = junk_element.get_coordinates();

                if let Some(next_tile) = board.get_from_coordinate_move((x, y), DirectionMove::from(direction))
                    && let Some(tile_directions) = next_tile.get_directions()
                    && tile_directions.contains(&direction.get_opposed())
                {} else {
                    continue 'outer;
                }

                (x, y) = move_tile(direction, x, y);


                while let Some(next_tile) = board.get_from_coordinate_move((x, y), DirectionMove::from(direction))
                    && let Some(tile_directions) = next_tile.get_directions()
                    && tile_directions.contains(&direction.get_opposed())
                {
                    if tile_directions.contains(&direction.turn_right()) {
                        blocking_tiles_found += 1;
                        println!("found blocking on lane at ({}, {})\n", next_tile.get_coordinates().0, next_tile.get_coordinates().1);
                    }

                    (x, y) = move_tile(direction, x, y);
                }

                // println!("got out of the lane at ({}, {})\n", x, y);

                while let Some(next_tile) =
                    board.get_from_coordinate_move((x, y), DirectionMove::from(direction))
                {
                    match next_tile.get_map_type() {
                        Junk => break,
                        Void(past_directions) | Guard(_, past_directions) => {
                            if past_directions.contains(&direction.turn_right()) {
                                blocking_tiles_found += 1;
                                println!("found blocking outside of lane at ({}, {})\n", next_tile.get_coordinates().0, next_tile.get_coordinates().1);
                            }

                            (x, y) = move_tile(direction, x, y);
                        }
                    }
                }
            }

            blocking_tiles_found
        })
        .sum();

    Ok((visited_tiles_count, blocking_count))
}

fn move_tile(guard_direction: GuardDirection, guard_x: usize, guard_y: usize) -> (usize, usize) {
    move_coordinates((guard_x, guard_y), guard_direction)
        .expect("Should find the next tile to move to")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solver() -> Result<(), Box<dyn Error>> {
        let path = format!("{PATH}/src/test");
        let (tiles_visited_count, blocking_count) = solve(&path)?;
        assert_eq!(41, tiles_visited_count);
        assert_eq!(6, blocking_count);

        Ok(())
    }

    #[test]
    fn test_solution() -> Result<(), Box<dyn Error>> {
        let path = format!("{PATH}/src/input");
        let (tiles_visited_count, blocking_count) = solve(&path)?;
        assert_eq!(4647, tiles_visited_count);
        assert!((231..).contains(&blocking_count));

        Ok(())
    }
}
