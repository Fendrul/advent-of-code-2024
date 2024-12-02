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
