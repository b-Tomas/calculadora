use std::{error::Error, ops::Index};

#[derive(Debug)]
pub struct Matrix {
    pub m: usize,
    pub n: usize,
    data: Vec<Vec<f32>>,
}

impl Clone for Matrix {
    fn clone(&self) -> Matrix {
        Matrix { m: self.m.clone(), n: self.n.clone(), data: self.data.clone() }
    }
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

    pub fn fill(&mut self, num: f32) {
        self.data = vec![vec![num; self.n]; self.n]
    }

    pub fn equals(&self, other: &Matrix) -> bool {
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

#[cfg(test)]
mod tests {
    use crate::structs::Matrix;

    fn create2by2() -> Matrix {
        return Matrix::new_from(2, 2, &[&[1.0, 2.0], &[3.0, 4.0]]).unwrap();
    }

    #[test]
    fn create_empty_matrix() {
        let rows: usize = 4;
        let cols: usize = 3;
        let matrix_a = Matrix::new_empty(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                assert_eq!(matrix_a[i][j], 0.0)
            }
        }
        assert_eq!(matrix_a.m, rows);
        assert_eq!(matrix_a.n, cols);
    }

    #[test]
    fn create_matrix_from_data() {
        let m = Matrix::new_from(2, 2, &[&[1.0, 2.0], &[3.0, 4.0]]).unwrap();
        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][1], 2.0);
        assert_eq!(m[1][0], 3.0);
        assert_eq!(m[1][1], 4.0);
    }

    #[test]
    fn is_squared() {
        assert!(create2by2().is_squared());
        assert!(!Matrix::new_from(1, 2, &[&[1.0, 2.0]]).unwrap().is_squared())
    }

    #[test]
    fn equals() {
        let m1 = create2by2();
        let m2 = create2by2();
        assert!(m1.equals(&m2));
        let m3 = Matrix::new_from(2, 3, &[&[1.0, 2.0, 3.0], &[3.0, 4.0, 3.0]]).unwrap();
        assert!(!m1.equals(&m3));
    }
}
