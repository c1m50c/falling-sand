use super::vector::Vector2U;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    /// **Fields:** `MaterialId`
    Solid(usize),
    Empty,
}


pub struct Cell {
    pub state: CellState,
    position: Vector2U,
}


impl Cell {
    #[inline(always)]
    pub const fn new(state: CellState, position: Vector2U) -> Self {
        return Self {
            state,
            position,
        }
    }

    #[inline(always)]
    pub fn get_position(&self) -> &Vector2U {
        return &self.position;
    }
}