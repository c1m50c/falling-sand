use fixed_vectors::Vector2;

#[derive(Debug, PartialEq)]
pub struct Cell {
    material: u32
}


impl Cell {
    /// Constructs a new [`Cell`].
    /// 
    /// # Example
    /// ```rust
    /// use crate::logic::Cell;
    /// 
    /// let cell = Cell::new(1);
    /// assert_eq!(cell, Cell { material: 1 });
    /// ```
    pub const fn new(material: u32) -> Self {
        return Self {
            material,
        };
    }
}


/// Converts a given `position` to the coresponding index within a matrix.
/// 
/// # Example
/// ```rust
/// use crate::logic::{Cell, position_to_index};
/// 
/// let mut grid = Vec::with_capacity(64 * 64);
/// for x in 0..64 {
///     for y in 0..64 {
///         grid.push(Cell { 0 });
///     }
/// }
/// 
/// let index = position_to_index(Vector2::new(1, 0), Vector2::new(64, 64));
/// let cell = &grid[index];
/// assert_eq!(cell, &grid[1]);
/// ```
pub const fn position_to_index(position: &Vector2<u32>, dimensions: &Vector2<u32>) -> usize {
    return (position.y + (position.x * dimensions.y)) as usize;
}