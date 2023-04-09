use crate::structs::Matrix;
use std::error::Error;

pub fn sum(ma: &Matrix, mb: &Matrix) -> Result<Matrix, Box<dyn Error>> {
    if ma.m != mb.m || ma.n != mb.n {
        return Err("bad dimensions")?;
    }
    let mut mc = Matrix::new_empty(ma.m, ma.n);
    for i in 0..ma.n {
        for j in 0..ma.m {
            mc.set(i, j, ma[i][j] + mb[i][j]);
        }
    }
    return Ok(mc);
}

pub fn sub(ma: &Matrix, mb: &Matrix) -> Result<Matrix, Box<dyn Error>> {
    if ma.m != mb.m || ma.n != mb.n {
        return Err("bad dimensions")?;
    }
    let mb = &mul_scalar(mb, -1.0);
    let mut mc = Matrix::new_empty(ma.m, ma.n);
    for i in 0..ma.n {
        for j in 0..ma.m {
            mc.set(i, j, ma[i][j] + mb[i][j]);
        }
    }
    return Ok(mc);
}

pub fn mul(m1: &Matrix, m2: &Matrix) -> Result<Matrix, Box<dyn Error>> {
    if m1.n != m2.m {
        return Err("Bad dimensions")?;
    };

    let mut res: Matrix = Matrix::new_empty(m1.m, m2.n);

    for i in 0..res.m {
        for j in 0..res.n {
            let mut value = 0.0;
            for k in 0..m1.n {
                value += m1[i][k] * m2[k][j];
            }
            res.set(i, j, value);
        }
    }

    return Ok(res);
}

pub fn mul_scalar(mat: &Matrix, num: f32) -> Matrix {
    let mut res = Matrix::new_empty(mat.m, mat.n);
    for i in 0..mat.m {
        for j in 0..mat.n {
            res.set(i, j, mat[i][j] * num);
        }
    }
    return res;
}

// Calcula el determinante de la matriz mediante el desarrollo por cofactores
pub fn det(m: &Matrix) -> Result<f32, Box<dyn Error>> {
    if !m.is_squared() || m.m == 0 || m.n == 0 {
        return Err("Bad dimensions")?;
    }

	fn _det_recursivo(m: &Matrix, hidden_rows: &Vec<bool>, hidden_cols: &Vec<bool>, sign_positive: bool) -> f32 {
		let mut sum = 0.0;
		let mut hr: Vec<bool> = hidden_rows.clone();
		let mut hc: Vec<bool> = hidden_cols.clone();
		let mut math_was_done = false;
		for i in 0..m.m {
			if hr[i] { continue; }
			for j in 0..m.n {
				if hc[j] { continue; }
				// Elijo esta esquina
				hr[i] = true;
				hc[j] = true;
				let inner_det: f32 = m[i][j] * _det_recursivo(m, &hr, &hc, !sign_positive);
				math_was_done = true;
				sum += inner_det * (if sign_positive {1.0} else {-1.0}) ;
			}
		} 
		if !math_was_done { return 1.0; }
		return sum;
	}

	return Ok(_det_recursivo(&m, &vec![false; m.n], &vec![false; m.m], true));
}

pub fn id_matrix(n: usize) -> Result<Matrix, Box<dyn Error>> {
    let mut res: Matrix = Matrix::new_empty(n, n);
    for i in 0..n {
        res.set(i, i, 1.0);
    }
    return Ok(res);
}

pub fn transp_squared_matrix(m: &Matrix) -> Result<Matrix, Box<dyn Error>> {
    if !m.is_squared() {
        return Err("Bad dimensions")?;
    }
    let mut res: Matrix = Matrix::new_empty(m.n, m.m);
    for i in 0..m.n {
        for j in 0..m.m {
            if i != j {
                res.set(j, i, m[i][j]);
            } else {
                res.set(i, j, m[i][j]);
            }
        }
    }
    return Ok(res);
}

pub fn inverse_ortogonal_matrix(m: &Matrix) -> Result<Matrix, Box<dyn Error>> {
    // NOTE: al dope
    if !m.is_squared() {
        return Err("Bad dimensions")?;
    }
    let mut res = Matrix::new_empty(m.n, m.m);
    for i in 0..m.n {
        for j in 0..m.m {
            if i != j {
                if m[i][j] == -m[j][i] {
                    res.set(j, i, m[i][j])
                } else {
                    return Err("Matrix is not ortogonal")?;
                }
            } else {
                if m[i][j] != 0.0 {
                    res.set(i, j, m[i][j]);
                }
            }
        }
    }
    return Ok(res);
}

#[cfg(test)]
mod tests {
    use crate::{math, structs::Matrix};

    fn create2by2() -> Matrix {
        return Matrix::new_from(2, 2, &[&[1.0, 2.0], &[3.0, 4.0]]).unwrap();
    }

    #[test]
    fn mul_mat_scalar() {
        let m = create2by2();
        let m2 = math::mul_scalar(&m, 2.0);

        assert_eq!(m2[0][0], 2.0);
        assert_eq!(m2[0][1], 4.0);
        assert_eq!(m2[1][0], 6.0);
        assert_eq!(m2[1][1], 8.0);
    }

    #[test]
    fn matrix_multiplication() {
        let m1 = Matrix::new_from(2, 3, &[&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]]).unwrap();
        let m2 = Matrix::new_from(2, 3, &[&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]]).unwrap();
        math::mul(&m1, &m2).unwrap_err(); // should fail (so, not fail because of unwrap_err)

        let m3 = Matrix::new_from(3, 1, &[&[1.0], &[2.0], &[3.0]]).unwrap();
        math::mul(&m3, &m1).unwrap_err(); // should fail
        let res = math::mul(&m1, &m3).unwrap(); // should be ok
        assert_eq!(res.m, 2);
        assert_eq!(res.n, 1);
        assert_eq!(res[0][0], 14.0);
        assert_eq!(res[1][0], 32.0);
    }

    #[test]
    fn matrix_sum() {
        let m1 = Matrix::new_from(2, 3, &[&[1.0, 2.0, 3.0], &[3.0, 4.0, 3.0]]).unwrap();
        let m2 = create2by2();
        let _err = math::sum(&m1, &m2).unwrap_err(); // This should be an error. Otherwise it will panic

        // Let's test an actual sum
        let m1 = create2by2();
        let m2 = create2by2();
        let res = math::sum(&m1, &m2).unwrap();

        assert_eq!(res[0][0], 2.0);
        assert_eq!(res[0][1], 4.0);
        assert_eq!(res[1][0], 6.0);
        assert_eq!(res[1][1], 8.0);
    }

    #[test]
    fn matrix_sub() {
        let m1 = create2by2();
        let m2 = create2by2();
        let res = math::sub(&m1, &m2).unwrap();

        assert_eq!(res[0][0], 0.0);
        assert_eq!(res[0][1], 0.0);
        assert_eq!(res[1][0], 0.0);
        assert_eq!(res[1][1], 0.0);
    }

    #[test]
    fn determinant() {
        let m = create2by2();
        assert_eq!(math::det(&m).unwrap(), -2.0);
    }

    #[test]
    fn orthogonal_test() {
        let m = Matrix::new_from(2, 2, &[&[1.0, -1.0], &[1.0, 1.0]]).unwrap();
        let res = math::inverse_ortogonal_matrix(&m).unwrap();

        assert_eq!(res[0][0], 1.0);
        assert_eq!(res[0][1], 1.0);
        assert_eq!(res[1][0], -1.0);
        assert_eq!(res[1][1], 1.0);
    }

    #[test] // test for identity matrix
    fn test_id() {
        let n: usize = 3;
        let res: Matrix = math::id_matrix(n).unwrap();

        assert_eq!(res[0][0], 1.0);
        assert_eq!(res[1][1], 1.0);
        assert_eq!(res[2][2], 1.0);
    }
    #[test] //test for transposed matrix
    fn test_trasp() {
        let m: Matrix = Matrix::new_from(2, 2, &[&[2.0, -1.0], &[3.0, 6.0]]).unwrap();
        let res: Matrix = math::transp_squared_matrix(&m).unwrap();

        assert_eq!(res[0][0], 2.0);
        assert_eq!(res[0][1], 3.0);
        assert_eq!(res[1][0], -1.0);
        assert_eq!(res[1][1], 6.0);
    }
}
