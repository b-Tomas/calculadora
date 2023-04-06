use std::{ops::Index, error::Error};

#[derive(Debug)]
pub struct Matrix {
    pub m: usize,
    pub n: usize,
    data: Vec<Vec<f32>>,
}

impl Matrix {
    pub fn new_empty(rows: usize, cols: usize) -> Matrix {
        Matrix {
            m: rows,
            n: cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn new_from(rows: usize, cols: usize, arr: &[&[f32]]) -> Result<Matrix, Box<dyn Error>> {
        if arr.len() != rows {
            return Err("Bad dimensions")?;
        }

        let mut data: Vec<Vec<f32>> = Vec::new();
        for a in arr {
            if a.len() != cols {
                return Err("Bad dimensions")?;
            }
            data.push(a.to_vec());
        }
        return Ok(Matrix {
            m: rows,
            n: cols,
            data,
        });
    }

    pub fn equals(&self, other: Matrix) -> bool {
        if self.m != other.m || self.n != other.n {
            return false;
        }
        for i in 0..self.m {
            for j in 0..self.n {
                if self[i][j] != other[i][j] {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn set(&mut self, m: usize, n: usize, value: f32) {
        self.data[m][n] = value;
    }

    pub fn is_squared(&self) -> bool {
        return self.m == self.n;
    }
}

// Implemetar acceso por indice y doble indice
impl Index<usize> for Matrix {
    type Output = Vec<f32>;

    fn index(&self, row_index: usize) -> &Self::Output {
        assert!(row_index < self.m);

        &self.data[row_index]
    }
}
