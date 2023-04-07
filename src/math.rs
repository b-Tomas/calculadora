use crate::{structs::Matrix};
use std::{error::Error};


pub fn sum(ma: &Matrix, mb: &Matrix) -> Result<Matrix, Box<dyn Error>> {
	if ma.m != mb.m || ma.n != mb.n {
		return Err("bad dimensions")?;
	}
	let mut mc = Matrix::new_empty(ma.m,ma.n);
	for i in 0..ma.n {
		for j in 0..ma.m {
            mc.set(i,j,ma[i][j]+mb[i][j]);
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
	if !m.is_squared() || m.m == 0 || m.n == 0 { return Err("Bad dimensions")?; }

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
//april 7 update
pub fn id_matrix(n:usize) -> Result<Matrix, Box<dyn Error>>{
	let mut res: Matrix=Matrix::new_empty(n, n);
	for i in 0..n {
			res.set(i,i,1.0);
		}
	return Ok(res);
}


pub fn trasp_squared_matrix (m: &Matrix) -> Result<Matrix, Box<dyn Error>>{
	if !m.is_squared() {  return Err("Bad dimensions")?;}
	let mut res: Matrix= Matrix::new_empty(m.n,m.m);
	for i in 0..m.n{
		for j in 0..m.m{
			if i!=j{
				res.set(j,i, m[i][j]);
			}
			else {
				res.set(i,j, m[i][j]);
			}
		}	
	}
	return Ok(res);
}


pub fn inverse_ortogonal_matrix(m: &Matrix) -> Result<Matrix, Box<dyn Error>> { // NOTE: al dope 
    if !m.is_squared() {
        return Err("Bad dimensions")?;
    }
    let mut res=Matrix::new_empty(m.n,m.m);
    for i in 0..m.n {
        for j in 0..m.m {
            if i!=j {
                if m[i][j] == -m[j][i]{
                res.set(j,i,m[i][j])
                }
                else {
                    return Err("Matrix is not ortogonal")?;
                }
            }
            else {
                if m[i][j] != 0.0 {
                    res.set(i,j,m[i][j]);
                }
            }
        }

    }
    return Ok(res);
}
