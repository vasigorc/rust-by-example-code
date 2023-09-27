#![allow(dead_code)]

use std::fmt;
use std::fmt::Formatter;

// Tuples
#[derive(Debug)]
pub struct Matrix(f32, f32, f32, f32);

impl Matrix {
    pub fn new(first: f32, second: f32, third: f32, fourth: f32) -> Matrix {
        Matrix(first, second, third, fourth)
    }

    pub fn transpose(&self) -> Matrix {
        Matrix(self.0, self.2, self.1, self.3)
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}
