use GuardDirection::{Down, Left, Right, Up};
use MapType::{Guard, Junk, Void};
use array_utils::DirectionMove;

#[derive(Debug)]
pub struct MapElement {
    map_type: MapType,
    x: usize,
    y: usize,
}
impl MapElement {
    pub fn new(map_type: char, x: usize, y: usize) -> Self {
        MapElement {
            map_type: MapType::from(map_type),
            x,
            y,
        }
    }

    pub fn get_map_type(&self) -> &MapType {
        &self.map_type
    }

    pub fn get_mut_map_type(&mut self) -> &mut MapType {
        &mut self.map_type
    }

    pub fn get_coordinates(&self) -> (usize, usize) {
        (self.x, self.y)
    }
    
    pub fn get_directions(&self) -> Option<&Vec<GuardDirection>> {
        self.map_type.get_directions()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MapType {
    Void(Vec<GuardDirection>), // the tile has already been visited or not
    Junk,
    Guard(GuardDirection, Vec<GuardDirection>),
}

impl MapType {
    pub fn get_directions(&self) -> Option<&Vec<GuardDirection>> {
        match self {
            Void(past_directions) | Guard(_, past_directions) => Some(past_directions),
            Junk => None,
        }
    }

    pub fn get_mut_directions(&mut self) -> Option<&mut Vec<GuardDirection>> {
        match self {
            Void(past_directions) | Guard(_, past_directions) => Some(past_directions),
            Junk => None,
        }
    }
}
impl From<char> for MapType {
    fn from(character: char) -> Self {
        match character {
            '.' => Void(Vec::with_capacity(4)),
            '#' => Junk,
            '^' | '>' | 'v' | '<' => {
                let direction = GuardDirection::from(character);
                Guard(direction, vec![direction])
            }
            _ => panic!("Invalid character to create a map element: {}", character),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum GuardDirection {
    Up,
    Down,
    Left,
    Right,
}

impl GuardDirection {
    pub fn turn_right(self) -> GuardDirection {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
    
    pub(crate) fn get_opposed(self) -> GuardDirection {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
    
    pub(crate) fn turn_left(self) -> GuardDirection {
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }
}

impl From<char> for GuardDirection {
    fn from(character: char) -> Self {
        match character {
            '^' => Up,
            'v' => Down,
            '<' => Left,
            '>' => Right,
            _ => panic!(
                "Invalid character to create a guard direction: {}",
                character
            ),
        }
    }
}

impl From<GuardDirection> for DirectionMove {
    fn from(guard_direction: GuardDirection) -> Self {
        match guard_direction {
            Up => DirectionMove::Up,
            Down => DirectionMove::Down,
            Left => DirectionMove::Left,
            Right => DirectionMove::Right,
        }
    }
}
