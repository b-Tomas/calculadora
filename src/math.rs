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
    return sum(ma, mb);
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

pub fn pow(mat: &Matrix, exp: i8) -> Result<Matrix, Box<dyn Error>> {
    if !mat.is_squared() {
        return Err("Bad dimensions")?;
    } else if exp == 0 {
        return Ok(id_matrix(mat.n));
    } else if exp == 1 {
        return Ok(mat.clone());
    }
    let mut res = mat.clone();
    for _ in 1..exp {
        if let Ok(_res) = mul(&res, &mat) {
            res = _res;
        } else {
            return Err("Bad dimensions")?;
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

fn _det_recursivo(m: &Matrix, hidden_rows: &Vec<bool>, hidden_cols: &Vec<bool>) -> f32 {
    let mut sum = 0.0;
    let mut sign_positive = true;
    let mut hr: Vec<bool> = hidden_rows.clone();
    let mut hc: Vec<bool> = hidden_cols.clone();
    
    for i in 0..m.m {
        if hr[i] { continue; }
        for j in 0..m.n {
            if hc[j] { continue; }
            // Si lo que queda es una submatriz 1x1, devolver ese valor
            if hr.iter().filter(|&&x| !x).count() == 1 && hc.iter().filter(|&&x| !x).count() == 1 {
                return m[i][j];
            }
            // Elijo esta celda para "tapar" fila y columna
            hr[i] = true;
            hc[j] = true;
            let inner_det = _det_recursivo(m, &hr, &hc);
            sum += m[i][j] * inner_det * (if sign_positive {1.0} else {-1.0}) ;
            sign_positive = !sign_positive;
            // Destapo antes de seguir
            hr[i] = false;
            hc[j] = false;
        }
        break;
    }
    return sum;
}

// Calcula el determinante de la matriz mediante el desarrollo por cofactores
pub fn det(m: &Matrix) -> Result<f32, Box<dyn Error>> {
    if !m.is_squared() || m.m == 0 || m.n == 0 {
        return Err("Bad dimensions")?;
    }
    return Ok(_det_recursivo(&m, &vec![false; m.n], &vec![false; m.m]));
}

pub fn id_matrix(n: usize) -> Matrix {
    let mut res: Matrix = Matrix::new_empty(n, n);
    for i in 0..n {
        res.set(i, i, 1.0);
    }
    return res;
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
pub fn adj_calculus(m:&Matrix) -> Result<Matrix, Box<dyn Error>>{
	if !m.is_squared(){
		return Err("Bad dimensions")?;
	}
	let mut sign:bool=true;
	let mut res:Matrix =Matrix::new_empty(m.n, m.m);
	for i in 0..m.n{
		for j in 0..m.m{
			let mut hidden_rows: Vec<bool> = vec![false; m.n];
			let mut hidden_columns: Vec<bool> = vec![false; m.m];
			hidden_rows[i]=true;
			hidden_columns[j]=true;
			res.set(i,j, (if sign {1.0} else {-1.0}*_det_recursivo(&m,&hidden_rows,&hidden_columns)));
			sign = !sign;
		}
	}
	return Ok(res);
}

pub fn inverse_matrix (m: &Matrix) -> Result<Matrix, Box<dyn Error>>{
	if !m.is_squared(){
		return Err("Bad dimensions")?;
	} else {
		let aux: f32 = det(&m).unwrap();
		if aux != 0.0 {
			let trasp: Matrix = transp_squared_matrix(&m).unwrap();
			let adj: Matrix = adj_calculus(&trasp).unwrap(); // Calculo el adjunto de la traspuesta
			let inverse: Matrix = mul_scalar(&adj, 1.0 / aux);
			return Ok(inverse);
		} else {
			return Err("No inverse")?;
		}
	}
}
#[cfg(test)]
mod tests {
    use crate::{math::{self, pow, id_matrix}, structs::Matrix};

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
        let m = Matrix::new_from(2, 2, &[&[6.0, 7.0], &[-2.0, 8.0]]).unwrap();
        assert_eq!(math::det(&m).unwrap(), 62.0);
        let m:Matrix=Matrix::new_from(3, 3, &[&[2.0, -1.0, 3.0], &[3.0, 6.0, 7.0], &[4.0, -2.0, 8.0]]).unwrap();
        assert_eq!(math::det(&m).unwrap(), 30.0);
        let m = create2by2();
        assert_eq!(math::det(&m).unwrap(), -2.0);
    }

	#[test]
	fn determinant2x2() {
        let m = Matrix::new_from(2,2,&[&[6.0,7.0], &[-2.0, 8.0]]).unwrap();
        assert_eq!(math::det(&m).unwrap(), 62.0);
    }

	#[test]
	fn determinant3x3(){
		let m:Matrix=Matrix::new_from(3, 3, &[&[2.0, -1.0, 3.0], &[3.0, 6.0, 7.0], &[4.0, -2.0, 8.0]]).unwrap();
		assert_eq!(math::det(&m).unwrap(), 30.0);
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

    #[test]
    fn test_id() {
        let n: usize = 3;
        let res: Matrix = math::id_matrix(n);
	    assert_eq!(res[0][0], 1.0);
        assert_eq!(res[1][1], 1.0);
        assert_eq!(res[2][2], 1.0);
    }

    #[test]
    fn test_trasp() {
        let m: Matrix = Matrix::new_from(2, 2, &[&[2.0, -1.0], &[3.0, 6.0]]).unwrap();
        let res: Matrix = math::transp_squared_matrix(&m).unwrap();
        
        assert_eq!(res[0][0], 2.0);
        assert_eq!(res[0][1], 3.0);
        assert_eq!(res[1][0], -1.0);
        assert_eq!(res[1][1], 6.0);
    }

    #[test]
    fn mat_pow() {
        let mat = create2by2();
        assert!(pow(&mat, 0).unwrap().equals(&id_matrix(2)));
        assert!(pow(&mat, 1).unwrap().equals(&mat));
        assert!(pow(&mat, 2).unwrap().equals(&Matrix::new_from(2, 2, &[&[7.0, 10.0], &[15.0, 22.0]]).unwrap()));
    }

	#[test] //test for ad matrix
	fn test_adj() {
		let m: Matrix = Matrix::new_from(3, 3, &[&[2.0, -1.0, 3.0], &[3.0, 6.0, 7.0], &[4.0, -2.0, 8.0]]).unwrap();
        let res: Matrix = math::adj_calculus(&m).unwrap();

		assert_eq!(res[0][0], 62.0);
        assert_eq!(res[0][1], 4.0);
        assert_eq!(res[0][2], -30.0);
        assert_eq!(res[1][0], 2.0);
		assert_eq!(res[1][1], 4.0);
		assert_eq!(res[1][2], 0.0);
		assert_eq!(res[2][0], -25.0);
		assert_eq!(res[2][1], -5.0);
		assert_eq!(res[2][2], 15.0);
	}

	#[test]

	fn inverse_test() {
		let m: Matrix = Matrix::new_from(3, 3, &[&[2.0, -1.0, 3.0], &[3.0, 6.0, 7.0], &[4.0, -2.0, 8.0]]).unwrap();
		let res: Matrix = math::inverse_matrix(&m).unwrap();
		let e:f32=0.0001;

		assert!((res[0][0]- 31.00/15.00).abs()<e);
        assert!((res[0][1] - 1.00/15.00).abs()<e);
        assert!((res[0][2] - -5.00/6.00).abs()<e);
        assert!((res[1][0] - 2.00/15.00).abs()<e);
		assert!((res[1][1] - 2.00/15.00).abs()<e);
		assert!((res[1][2] - -1.00/6.00).abs()<e);
		assert!((res[2][0] - -1.00).abs()<e);
		assert!((res[2][1] - 0.0).abs()<e);
		assert!((res[2][2] - 1.00/2.00).abs()<e);
	}
}