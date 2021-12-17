use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Div, DivAssign};
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector2U {
    pub x: u32,
    pub y: u32,
}


#[wasm_bindgen]
impl Vector2U {
    pub fn new(x: u32, y: u32) -> Self {
        return Self {
            x,
            y,
        };
    }
}


impl Add for Vector2U {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        return Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


impl AddAssign for Vector2U {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}


impl Sub for Vector2U {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        return Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}


impl SubAssign for Vector2U {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}


impl Mul for Vector2U {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        return Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}


impl MulAssign for Vector2U {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
        };
    }
}


impl Div for Vector2U {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        return Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}


impl DivAssign for Vector2U {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
        };
    }
}