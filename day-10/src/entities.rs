use DirectionMove::{Down, Left, Right, Up};
use array_utils::{DirectionMove, TableUtils};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Deref;

pub struct Map {
    map: Vec<Vec<MapCell>>,
}

impl Deref for Map {
    type Target = Vec<Vec<MapCell>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl Map {
    pub fn new(map: Vec<Vec<MapCell>>) -> Self {
        Self { map }
    }

    pub fn give_starting_ghosts(&self) -> Vec<Ghost> {
        self.map
            .iter()
            .flat_map(|row| row.iter())
            .filter(|cell| cell.value == 0)
            .map(|cell| Ghost::new(cell.value, cell.coords, self))
            .collect()
    }
}



pub struct MapCell {
    value: usize,
    coords: (usize, usize),
}

impl MapCell {
    pub fn new(value: usize, coords: (usize, usize)) -> Self {
        Self { value, coords }
    }
}



#[derive(Clone)]
pub struct Ghost<'a> {
    value: usize,
    coords: (usize, usize),
    map_ref: &'a Map,
}

impl PartialEq for Ghost<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.coords == other.coords
    }
}

impl Eq for Ghost<'_> {}

impl Hash for Ghost<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.coords.hash(state);
    }
}

impl<'a> Ghost<'a> {
    pub fn new(value: usize, coords: (usize, usize), map: &'a Map) -> Self {
        Self {
            value,
            coords,
            map_ref: map,
        }
    }

    pub fn new_from_cell(cell: &MapCell, map: &'a Map) -> Self {
        Self {
            value: cell.value,
            coords: cell.coords,
            map_ref: map,
        }
    }

    pub fn explore(self) -> Box<dyn Iterator<Item = Ghost<'a>> + 'a> {
        if self.value == 9 {
            return Box::new(std::iter::once(self));
        }

        let (x, y) = self.coords;
        Box::new(
            [
                self.map_ref.get_from_coordinate_move((x, y), Up),
                self.map_ref.get_from_coordinate_move((x, y), Down),
                self.map_ref.get_from_coordinate_move((x, y), Left),
                self.map_ref.get_from_coordinate_move((x, y), Right),
            ]
            .into_iter()
            .filter_map(move |cell| {
                if let Some(cell) = cell
                    && cell.value == self.value + 1
                {
                    return Some(Ghost::new_from_cell(cell, self.map_ref));
                }

                None
            })
            .flat_map(|ghost| ghost.explore()),
        )
    }
}

impl Debug for Ghost<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ghost")
            .field("value", &self.value)
            .field("coords", &self.coords)
            .finish()
    }
}
