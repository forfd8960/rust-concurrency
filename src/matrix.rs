use anyhow::{anyhow, Ok, Result};
use std::fmt;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul};

use crate::vector::Vector;

pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Matrix<T>
where
    T: Debug,
{
    pub fn new(data: impl Into<Vec<T>>, rows: usize, cols: usize) -> Self {
        Self {
            data: data.into(),
            rows,
            cols,
        }
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;

        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{:?}", self.data[i * self.cols + j])?;
                if j != self.cols - 1 {
                    write!(f, " ")?;
                }
            }
            if i != self.rows - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;

        std::result::Result::Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Display + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix({}, {}),: {}", self.rows, self.cols, self)
    }
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Default + Copy + Add<Output = T> + AddAssign + Mul<Output = T> + std::iter::Sum,
{
    assert_eq!(a.len(), b.len(), "a and b must be of the same length");

    Ok(a.iter().zip(b.iter()).map(|(x, y)| *x * *y).sum())
}

pub fn multiply<T>(a: Matrix<T>, b: Matrix<T>) -> Result<Matrix<T>>
where
    T: Debug + Default + Copy + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.cols != b.rows {
        return Err(anyhow!("matrix multiply err: a.cols != b.rows"));
    }

    let mut data: Vec<T> = vec![T::default(); a.rows * b.cols];
    for i in 0..a.rows {
        for j in 0..b.cols {
            let mut sum = T::default();
            for k in 0..a.cols {
                sum += a.data[i * a.cols + k] * b.data[k * b.cols + j];
            }
            data[i * b.cols + j] = sum;
        }
    }

    Ok(Matrix {
        data,
        rows: a.rows,
        cols: b.cols,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiply() -> anyhow::Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        let c = multiply(a, b)?;
        assert_eq!(c.rows, 2);
        assert_eq!(c.cols, 2);
        assert_eq!(c.data, vec![22, 28, 49, 64]);
        Ok(())
    }

    #[test]
    fn test_dot_product() -> anyhow::Result<()> {
        let a = Vector::new(vec![1, 2]);
        let b = Vector::new(vec![9, 8]);
        let c = dot_product(a, b)?;
        assert_eq!(c, 25);
        Ok(())
    }
}
