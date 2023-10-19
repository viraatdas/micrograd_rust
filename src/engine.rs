use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
pub struct Value {
    pub data: f64,
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            data: self.data + other.data,
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            data: self.data * other.data,
        }
    }
}

impl Value {
    pub fn new(data: f64) -> Self {
        Self { data }
    }
}
