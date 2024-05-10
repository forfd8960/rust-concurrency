use anyhow::{anyhow, Ok, Result};
use std::ops::{Add, AddAssign, Mul};
use std::{fmt, sync::mpsc};
use std::{fmt::Debug, thread};

use crate::{dot_product, vector::Vector};

const NUM_THREADS: usize = 4;

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

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

impl<T> MsgInput<T> {
    fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

impl<T> Msg<T> {
    fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}

pub fn multiply<T>(a: Matrix<T>, b: Matrix<T>) -> Result<Matrix<T>>
where
    T: Debug + Default + Copy + Add<Output = T> + AddAssign + Mul<Output = T> + std::iter::Sum,
{
    if a.cols != b.rows {
        return Err(anyhow!("matrix multiply err: a.cols != b.rows"));
    }

    let mut data: Vec<T> = vec![T::default(); a.rows * b.cols];
    for i in 0..a.rows {
        for j in 0..b.cols {
            let row = Vector::new(a.data[i * a.cols..(i + 1) * a.cols].to_vec());
            let col_data = b.data[j..]
                .iter()
                .step_by(b.cols)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);
            data[i * b.cols + j] += dot_product(row, col)?;
        }
    }

    Ok(Matrix {
        data,
        rows: a.rows,
        cols: b.cols,
    })
}

pub fn multiply_v1<T>(a: Matrix<T>, b: Matrix<T>) -> Result<Matrix<T>>
where
    T: Debug
        + Default
        + Copy
        + Add<Output = T>
        + AddAssign
        + Mul<Output = T>
        + std::iter::Sum
        + Send
        + 'static,
{
    if a.cols != b.rows {
        return Err(anyhow!("matrix multiply err: a.cols != b.rows"));
    }

    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        value,
                    }) {
                        eprintln!("Send error: {}", e);
                    };
                }
                anyhow::Ok(())
            });
            tx
        })
        .collect::<Vec<_>>();

    let matrix_cap = a.rows * b.cols;
    let mut data: Vec<T> = vec![T::default(); matrix_cap];
    let mut receivers: Vec<oneshot::Receiver<MsgOutput<T>>> = Vec::with_capacity(matrix_cap);

    for i in 0..a.rows {
        for j in 0..b.cols {
            let row = Vector::new(a.data[i * a.cols..(i + 1) * a.cols].to_vec());
            let col_data = b.data[j..]
                .iter()
                .step_by(b.cols)
                .copied()
                .collect::<Vec<_>>();

            let idx = i * b.cols + j;
            let col = Vector::new(col_data);
            let input = MsgInput::new(idx, row, col);

            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("Send error: {}", e);
            }

            receivers.push(rx);
        }
    }

    for receiver in receivers {
        let rev_msg_output = receiver.recv()?;
        data[rev_msg_output.idx] = rev_msg_output.value
    }

    Ok(Matrix {
        data,
        rows: a.rows,
        cols: b.cols,
    })
}

impl<T> Mul for Matrix<T>
where
    T: Debug
        + Default
        + Copy
        + Add<Output = T>
        + AddAssign
        + Mul<Output = T>
        + std::iter::Sum
        + Send
        + 'static,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        multiply_v1(self, rhs).expect("Expect matrix")
    }
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
    fn test_multiply_v1() -> anyhow::Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        let c = multiply_v1(a, b)?;
        assert_eq!(c.rows, 2);
        assert_eq!(c.cols, 2);
        assert_eq!(c.data, vec![22, 28, 49, 64]);
        Ok(())
    }

    #[test]
    fn test_multiply_should_error() {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 2, 2);
        let c = multiply_v1(a, b);
        assert!(c.is_err())
    }

    #[test]
    #[should_panic]
    fn test_multiply_should_panic() {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 2, 2);
        let _ = a * b;
    }

    #[test]
    fn test_multiply_matrix() -> anyhow::Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        let c = a * b;
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
