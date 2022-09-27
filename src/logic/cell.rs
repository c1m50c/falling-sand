#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cell {
    pub material: u32,
}


impl Cell {
    /// Constructs a new [`Cell`].
    /// 
    /// # Example
    /// ```rust
    /// use crate::logic::Cell;
    /// 
    /// let cell = Cell::new(0);
    /// assert_eq!(cell.material, 0);
    /// ```
    pub const fn new(material: u32) -> Self {
        return Self {
            material,
        };
    }
}