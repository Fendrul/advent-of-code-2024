pub mod coordinate;

use crate::coordinate::Coordinates;
use std::iter::{Skip, Zip};
use std::slice::Iter;

/// A trait that provides utility functions for working with array-like types containing elements of type T.
///
/// This trait implements operations that can be performed on sequences of elements
pub trait ArrayUtils<T> {
    /// Creates an iterator that yields pairs of elements from the array with a specified interval.
    ///
    /// # Arguments
    /// * `interval` - The number of elements to skip between each pair.
    ///
    /// # Returns
    /// Returns a `Zip` iterator that pairs each element with another element that is `interval` positions ahead.
    ///
    /// # Examples
    /// ```rust
    /// use array_utils::ArrayUtils;
    ///
    /// let array = vec![1, 2, 3, 4, 5];
    /// let pairs = array.pairwise(1);
    /// // This will iterate over: (1,2), (2,3), (3,4), (4,5)
    ///
    /// for (first, second) in pairs {
    ///     println!("Pair: {} {}", first, second);
    /// }
    /// ```
    fn pairwise(&self, interval: usize) -> Zip<Iter<'_, T>, Skip<Iter<'_, T>>>;
}

impl<T> ArrayUtils<T> for [T] {
    fn pairwise(&self, interval: usize) -> Zip<Iter<'_, T>, Skip<Iter<'_, T>>> {
        self.iter().zip(self.iter().skip(interval))
    }
}

pub trait TableUtils<T> {
    fn iter_table_with_coordinates<'a>(&'a self) -> impl Iterator<Item = (&'a T, Coordinates)>
    where
        T: 'a;

    fn get_from_coordinates(&self, i: usize, j: usize) -> Option<&T>;

    fn get_from_coordinate_move<U: Into<Coordinates>>(
        &self,
        coordinate: U,
        direction: DirectionMove,
    ) -> Option<&T>;
}

impl<T> TableUtils<T> for Vec<Vec<T>> {
    fn iter_table_with_coordinates<'a>(&'a self) -> impl Iterator<Item = (&'a T, Coordinates)>
    where
        T: 'a,
    {
        let width = self.len();
        let height = self[0].len();
        (0..width).flat_map(move |y| (0..height).map(move |x| (&self[y][x], Coordinates { x, y })))
    }

    fn get_from_coordinates(&self, x: usize, y: usize) -> Option<&T> {
        self.get(y).and_then(|row| row.get(x))
    }

    fn get_from_coordinate_move<U: Into<Coordinates>>(
        &self,
        coordinate: U,
        direction: DirectionMove,
    ) -> Option<&T> {
        let coordinate = coordinate.into();
        let (new_x, new_y) = move_coordinates(&coordinate, direction)?;

        self.get(new_y).and_then(|row| row.get(new_x))
    }
}

impl<T> TableUtils<T> for [&[T]] {
    fn iter_table_with_coordinates<'a>(&'a self) -> impl Iterator<Item = (&'a T, Coordinates)>
    where
        T: 'a,
    {
        let width = self.len();
        let height = self[0].len();
        (0..width).flat_map(move |y| (0..height).map(move |x| (&self[y][x], Coordinates { x, y })))
    }

    fn get_from_coordinates(&self, x: usize, y: usize) -> Option<&T> {
        self.get(y).and_then(|row| row.get(x))
    }

    fn get_from_coordinate_move<U: Into<Coordinates>>(
        &self,
        coordinate: U,
        direction: DirectionMove,
    ) -> Option<&T> {
        let coordinate = coordinate.into();
        let (new_x, new_y) = move_coordinates(&coordinate, direction)?;

        self.get(new_y).and_then(|row| row.get(new_x))
    }
}

fn move_coordinates(coordinate: &Coordinates, direction: DirectionMove) -> Option<(usize, usize)> {
    let (x, y) = coordinate.to_tuple();
    let (dx, dy) = direction.get_direction();
    let new_x = x.checked_add_signed(dx);
    let new_y = y.checked_add_signed(dy);

    let (new_x, new_y) = match (new_x, new_y) {
        (Some(x), Some(y)) => (x, y),
        _ => return None,
    };
    Some((new_x, new_y))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectionMove {
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl DirectionMove {
    pub fn get_direction(&self) -> (isize, isize) {
        match self {
            DirectionMove::Up => (0, -1),
            DirectionMove::Down => (0, 1),
            DirectionMove::UpLeft => (-1, -1),
            DirectionMove::UpRight => (1, -1),
            DirectionMove::DownLeft => (-1, 1),
            DirectionMove::DownRight => (1, 1),
        }
    }

    pub fn is_opposed_to(&self, other: DirectionMove) -> bool {
        let (dx, dy) = self.get_direction();
        let (dx_other, dy_other) = other.get_direction();

        dx == -dx_other && dy == -dy_other
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pairwise() {
        let array = [1, 2, 3, 4, 5];
        let pairs = array.pairwise(1);

        let expected = [(1, 2), (2, 3), (3, 4), (4, 5)];

        for ((first, second), expected) in pairs.zip(expected.iter()) {
            assert_eq!(*expected, (*first, *second));
        }
    }

    #[test]
    fn test_iter_table_with_coordinates() {
        let table = vec![vec![1, 2], vec![3, 4]];
        let mut iter = table.iter_table_with_coordinates();

        assert_eq!(Some((&1, Coordinates { x: 0, y: 0 })), iter.next());
        assert_eq!(Some((&2, Coordinates { x: 0, y: 1 })), iter.next());
        assert_eq!(Some((&3, Coordinates { x: 1, y: 0 })), iter.next());
        assert_eq!(Some((&4, Coordinates { x: 1, y: 1 })), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_get_from_coordinates() {
        let table = vec![vec![1, 2], vec![3, 4]];

        assert_eq!(Some(&1), table.get_from_coordinates(0, 0));
        assert_eq!(Some(&2), table.get_from_coordinates(1, 0));
        assert_eq!(Some(&3), table.get_from_coordinates(0, 1));
        assert_eq!(Some(&4), table.get_from_coordinates(1, 1));
        assert_eq!(None, table.get_from_coordinates(2, 0));
        assert_eq!(None, table.get_from_coordinates(0, 2));
    }

    #[test]
    fn test_get_from_coordinate_move() {
        let table = vec![vec![1, 2], vec![3, 4]];

        assert_eq!(
            Some(&1),
            table.get_from_coordinate_move((1, 1), DirectionMove::UpLeft)
        );
        assert_eq!(
            None,
            table.get_from_coordinate_move((1, 0), DirectionMove::UpLeft)
        );
        assert_eq!(
            Some(&2),
            table.get_from_coordinate_move((0, 1), DirectionMove::UpRight)
        );
        assert_eq!(
            None,
            table.get_from_coordinate_move((0, 0), DirectionMove::UpLeft)
        );
    }
}
