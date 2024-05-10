use std::ops::Deref;
use std::ops::{Add, AddAssign, Mul};

use anyhow::Result;

pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Default + Copy + Add<Output = T> + AddAssign + Mul<Output = T> + std::iter::Sum,
{
    assert_eq!(a.len(), b.len(), "a and b must be of the same length");

    Ok(a.iter().zip(b.iter()).map(|(x, y)| *x * *y).sum())
}
