use std::ops::{Index, IndexMut};
use fixed_vectors::Vector2;
use super::cell::Cell;


#[derive(Debug, PartialEq)]
pub struct CellChunk {
    cells: Vec<Cell>,
}


impl CellChunk {
    pub const DIMENSIONS: Vector2<u32> = Vector2::new(32, 32);

    /// Constructs a new [`CellChunk`] containing air cells.
    /// 
    /// # Example
    /// ```rust
    /// use crate::logic::CellChunk;
    /// 
    /// let chunk = CellChunk::new();
    /// assert_eq!(chunk[Vector2::new(0, 0)].material, 0);
    /// ```
    pub fn new() -> Self {
        let mut vec = Vec::with_capacity(Self::len());
        (0..Self::len())
            .for_each(|_| vec.push(Cell::new(0)));
        
        return Self {
            cells: vec,
        };
    }

    /// Returns the number of [`Cell`]s within the [`CellChunk`].
    /// This number should be equivalent to `DIMENSIONS.x * DIMENSIONS.y`.
    /// 
    /// # Example
    /// ```rust
    /// use crate::logic::CellChunk;
    /// 
    /// assert_eq!(
    ///     CellChunk::len(),
    ///     (CellChunk::DIMENSIONS.x * CellChunk::DIMENSIONS.y) as usize
    /// );
    /// ```
    pub const fn len() -> usize {
        return (Self::DIMENSIONS.x * Self::DIMENSIONS.y) as usize;
    }

    /// Returns the assumed index of the `position` within the `cells` field.
    /// This functionality is primarily used in the [`Index`] trait's implementation for [`CellChunk`].
    /// 
    /// # Example
    /// ```rust
    /// use crate::logic::CellChunk;
    /// 
    /// assert_eq!(CellChunk::position_as_index(Vector2::new(0, 0)), 0);
    /// ```
    pub const fn position_as_index(position: Vector2<u32>) -> usize {
        return (position.y + (position.x * Self::DIMENSIONS.y)) as usize;
    }
}


impl Index<Vector2<u32>> for CellChunk {
    type Output = Cell;

    fn index(&self, index: Vector2<u32>) -> &Self::Output {
        let index = Self::position_as_index(index);
        return &self.cells[index];
    }
}


impl IndexMut<Vector2<u32>> for CellChunk {
    fn index_mut(&mut self, index: Vector2<u32>) -> &mut Self::Output {
        let index = Self::position_as_index(index);
        return &mut self.cells[index];
    }
}