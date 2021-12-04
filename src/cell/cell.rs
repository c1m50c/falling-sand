use super::Vector2U;

/// For use within `CellChunk`s, which are used to simulate the Falling Sand Cellular Automata.
/// 
/// ## Fields
/// ```rs
/// pub material_id: u32 // ID of the Material for the `Cell`.
/// position: Vector2U // Relative Position of the `Cell` within a `CellChunk`.
/// ```
pub struct Cell {
    pub material_id: u32,
    position: Vector2U,
}


impl Cell {
    #[inline]
    pub fn new(material_id: u32, position: Vector2U) -> Self {
        return Self {
            material_id,
            position
        };
    }

    #[inline]
    pub fn get_position(&self) -> &Vector2U {
        return &self.position;
    }
}