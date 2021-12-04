use super::cell::Cell;
use super::Vector2U;
use std::vec::Vec;


/// Struct for containing `Cell`s.
/// 
/// ## Fields
/// ```rs
/// pub size: Vector2U // The `cells` Vector will corespond to this field.
/// cells: Vec<Cell> // Vector containing the `Cell`s of the `CellChunk`.
/// ```
pub struct CellChunk {
    pub size: Vector2U,
    cells: Vec<Cell>,
}


impl CellChunk {
    /// Returns the `index` of the `Cell` at the specified `position` within the `CellChunk`.
    /// 
    /// ## Parameters
    /// ```rs
    /// position: Vector2U // Position of the `Cell` relative to the `CellChunk`.
    /// size: Vector2U // Size of the `CellChunk`.
    /// ```
    #[inline]
    fn position_index(position: Vector2U, size: Vector2U) -> usize {
        return (position.1 + (position.0 * size.1)) as usize;
    }

    /// Returns `true` if the specified `position` is valid within the `size` extents.
    /// 
    /// ## Parameters
    /// ```rs
    /// position: Vector2U // Position of the `Cell` relative to the `CellChunk`.
    /// size: Vector2U // Size of the `CellChunk`
    /// ```
    #[inline]
    fn position_in_bounds(position: Vector2U, size: Vector2U) -> bool {
        if position.0 > size.0 - 1 || position.1 > size.1 - 1 { return false; }
        return true;
    }
}


impl CellChunk {
    #[inline]
    pub fn new(size: Vector2U) -> Self {
        let mut cells: Vec<Cell> = Vec::with_capacity((size.0 * size.1) as usize);
        
        for x in 0 .. size.0 {
            for y in 0 .. size.1 {
                let new_cell: Cell = Cell::new(0, (x, y));
                cells.push(new_cell);
            }
        }

        return Self {
            size,
            cells,
        };
    }

    /// Places a `Cell` at the specified `position` with the material of `m_id`.
    /// 
    /// ## Parameters
    /// ```rs
    /// position: Vector2U // Position of the `Cell` relative to the `CellChunk`.
    /// m_id: u32 // Material ID of the new `Cell`.
    /// ```
    #[inline]
    pub fn place_cell(&mut self, position: Vector2U, m_id: u32) {
        self.cells[Self::position_index(position, self.size)].material_id = m_id;
    }

    /// Removes a `Cell` at the specified `position`, replacing its `material_id` with zero.
    /// 
    /// ## Parameters
    /// ```rs
    /// position: Vector2U // Position of the `Cell` relative to the `CellChunk`.
    /// ```
    #[inline]
    pub fn remove_cell(&mut self, position: Vector2U) {
        self.cells[Self::position_index(position, self.size)].material_id = 0;
    }
}