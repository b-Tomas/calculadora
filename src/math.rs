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

// Calcula el determinante de la matriz mediante el desarrollo por cofactores
pub fn det(m: &Matrix) -> Result<f32, Box<dyn Error>> {
    if !m.is_squared() || m.m == 0 || m.n == 0 {
        return Err("Bad dimensions")?;
    }
    return Ok(_det_recursivo(&m, &vec![false; m.n], &vec![false; m.m]));
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

pub fn id_matrix(n: usize) -> Matrix {
    let mut res: Matrix = Matrix::new_empty(n, n);
    for i in 0..n {
        res.set(i, i, 1.0);
    }
    return res;
}

pub fn transpose(m: &Matrix) -> Result<Matrix, Box<dyn Error>> {
    let mut res: Matrix = Matrix::new_empty(m.n, m.m);
    for i in 0..m.m {
        for j in 0..m.n {
            res.set(j, i, m[i][j]);
        }
    }
    return Ok(res);
}

// Matriz de cofactores (sin trasponer)
pub fn adj(m:&Matrix) -> Result<Matrix, Box<dyn Error>>{
    if !m.is_squared() {
        return Err("Bad dimensions")?;
    }
    let mut sign: bool = true;
    let mut res: Matrix = Matrix::new_empty(m.n, m.m);
    for i in 0..m.m {
        for j in 0..m.n {
            let mut hidden_rows: Vec<bool> = vec![false; m.m];
            let mut hidden_columns: Vec<bool> = vec![false; m.n];
            hidden_rows[i] = true;
            hidden_columns[j] = true;
            res.set(i,j, (if sign {1.0} else {-1.0} * _det_recursivo(&m, &hidden_rows, &hidden_columns)));
            sign = !sign;
        }
        if m.m % 2 == 0 {
            sign = !sign;
        }
    }
    return Ok(res);
}

pub fn inv(m: &Matrix) -> Result<Matrix, Box<dyn Error>>{
    if !m.is_squared(){
        return Err("Bad dimensions")?;
    } else {
        let aux:f32 = det(&m).unwrap();
        if aux != 0.0 {
            let trasp: Matrix = transpose(&m).unwrap();
            let adj: Matrix = adj(&trasp).unwrap(); // Calculo el adjunto de la traspuesta
            let inverse: Matrix = mul_scalar(&adj, 1.0 / aux);
            return Ok(inverse);
        } else {
            return Err("No inverse")?;
        }
    }
}
fn data_loading(m: &Matrix, results: &Matrix) -> Matrix {
    let mut res: Matrix = Matrix::new_empty(m.n, m.m+1);
    for i in 0..m.n {
        for j in 0..m.m {
            res.set(i,j,m[i][j]);
        }
        res.set(i,res.m, results[i][0]);
    }
    return res;
}  

fn swap_rows(m: &mut Matrix, i: usize, j: usize) {
    let n_cols = m[0].len();
    let mut temp = vec![0.0; n_cols];
    for col_index in 0..n_cols {
        temp[col_index] = m[j][col_index];
        m.set( j, col_index, m[i][col_index]);
        m.set( i, col_index, temp[col_index]);
    }
}

#[derive(PartialEq, Eq)]
pub enum Compatibility {
    CompatibleDeterminado,
    CompatibleIndeterminado,
    Incompatible,
}

impl Compatibility {
    /// Returns `true` if the compatibility is [`CompatibleDeterminado`].
    ///
    /// [`CompatibleDeterminado`]: Compatibility::CompatibleDeterminado
    #[must_use]
    pub fn is_compatible_determinado(&self) -> bool {
        matches!(self, Self::CompatibleDeterminado)
    }

    /// Returns `true` if the compatibility is [`CompatibleIndeterminado`].
    ///
    /// [`CompatibleIndeterminado`]: Compatibility::CompatibleIndeterminado
    #[must_use]
    pub fn is_compatible_indeterminado(&self) -> bool {
        matches!(self, Self::CompatibleIndeterminado)
    }

    /// Returns `true` if the compatibility is [`Incompatible`].
    ///
    /// [`Incompatible`]: Compatibility::Incompatible
    #[must_use]
    pub fn is_incompatible(&self) -> bool {
        matches!(self, Self::Incompatible)
    }
}

// toma la matriz aumentada
pub fn solve_system(matrix: &Matrix) -> Compatibility {
    let m = matrix.m;
    let n = matrix.n;
    let mut matrix = matrix.clone();

    // Paso 1: Escalonar la matriz aumentada
    let mut i = 0;
    let mut j = 0;
    while i < m && j < n {
        // Buscar el valor máximo en la columna j
        let mut max_row = i;
        for k in i+1..m {
            if matrix[k][j].abs() > matrix[max_row][j].abs() {
                max_row = k;
            }
        }

        // Intercambiar filas para tener el valor máximo en la posición (i, j)
        if max_row != i {
            swap_rows(&mut matrix, i, max_row);
        }
        // Si el valor máximo en la posición (i, j) es cero, entonces toda la columna es cero
        if matrix[i][j] == 0.0 {
            j += 1;
            continue;
        }

        // Escalonar la fila i para tener un 1 en la posición (i, j)
        let pivot = matrix[i][j];
        for k in j..n {
            matrix.set(i, k,  matrix[i][k] / pivot);
        }

        // Restar la fila i a las filas siguientes para tener ceros debajo del pivote
        for k in i+1..m {
            let factor = matrix[k][j];
            for l in j..n {
                matrix.set(k, l, matrix[k][l] - factor * matrix[i][l]);
            }
        }

        i += 1;
        j += 1;
    }

    // Paso 2: Determinar la compatibilidad del sistema
    let mut rank = m;
    for i in (0..m).rev() {
        let mut nonzero = false;
        for j in 0..n-1 {
            if matrix[i][j] != 0.0 {
                nonzero = true;
                break;
            }
        }
        if !nonzero {
            if matrix[i][n-1] != 0.0 {
                return Compatibility::Incompatible;
            }
            rank -= 1;
        }
    }
    if rank < n-1 {
        return Compatibility::CompatibleIndeterminado;
    }

    // Paso 3: Resolver el sistema mediante sustitución hacia atrás
    let mut x = Matrix::new_empty(n-1, 1);
    for i in (0..n-1).rev() {
        let mut sum = 0.0;
        for j in i+1..n-1 {
            sum += matrix[i][j] * x[j][0];
        }
        x.set(i,0 , matrix[i][n-1] - sum);
    }
    Compatibility::CompatibleDeterminado
}

#[cfg(test)]
mod tests {
    use crate::structs::Matrix;
    use crate::math;

    fn create2by2() -> Matrix {
        return Matrix::new_from(2, 2, &[&[1.0, 2.0], &[3.0, 4.0]]).unwrap();
    }

    #[test]
    fn multiplication_by_scalar() {
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
    fn identity_matrix() {
        let n: usize = 3;
        let res: Matrix = math::id_matrix(n);
        assert_eq!(res[0][0], 1.0);
        assert_eq!(res[1][1], 1.0);
        assert_eq!(res[2][2], 1.0);
    }

    #[test]
    fn transposed() {
        let matrix: Matrix = Matrix::new_from(3, 2, &[&[1.0, 2.0], &[3.0, 4.0], &[5.0, 6.0]]).unwrap();
        let result: Matrix = math::transpose(&matrix).unwrap();
        let expected: Matrix = Matrix::new_from(2, 3, &[&[1.0, 3.0, 5.0], &[2.0, 4.0, 6.0]]).unwrap();
        assert!(expected.equals(&result));
    }

    #[test]
    fn mat_pow() {
        let mat = create2by2();
        assert!(math::pow(&mat, 0).unwrap().equals(&math::id_matrix(2)));
        assert!(math::pow(&mat, 1).unwrap().equals(&mat));
        assert!(math::pow(&mat, 2).unwrap().equals(&Matrix::new_from(2, 2, &[&[7.0, 10.0], &[15.0, 22.0]]).unwrap()));
    }

    #[test]
    fn test_adj() {
        let m: Matrix = Matrix::new_from(3, 3, &[&[2.0, -1.0, 3.0], &[3.0, 6.0, 7.0], &[4.0, -2.0, 8.0]]).unwrap();
        let res: Matrix = math::adj(&m).unwrap();

        assert_eq!(res[0][0], 62.0);
        assert_eq!(res[0][1], 4.0);
        assert_eq!(res[0][2], -30.0);
        assert_eq!(res[1][0], 2.0);
        assert_eq!(res[1][1], 4.0);
        assert_eq!(res[1][2], 0.0);
        assert_eq!(res[2][0], -25.0);
        assert_eq!(res[2][1], -5.0);
        assert_eq!(res[2][2], 15.0);

        let m = Matrix::new_from(2, 2, &[&[7.0, 10.0], &[15.0, 20.0]]).unwrap();
        let res = math::adj(&m).unwrap();
        let expected = Matrix::new_from(2, 2, &[&[20.0, -15.0], &[-10.0, 7.0]]).unwrap();
        assert!(res.equals(&expected));
    }

    #[test]
    fn inverse_test() {
        let m: Matrix = Matrix::new_from(3, 3, &[&[2.0, -1.0, 3.0], &[3.0, 6.0, 7.0], &[4.0, -2.0, 8.0]]).unwrap();
        let res: Matrix = math::inv(&m).unwrap();
        let e:f32=0.0001;

        assert!((res[0][0] - 31.00/15.00).abs()<e);
        assert!((res[0][1] - 1.00/15.00).abs()<e);
        assert!((res[0][2] - -5.00/6.00).abs()<e);
        assert!((res[1][0] - 2.00/15.00).abs()<e);
        assert!((res[1][1] - 2.00/15.00).abs()<e);
        assert!((res[1][2] - -1.00/6.00).abs()<e);
        assert!((res[2][0] - -1.00).abs()<e);
        assert!((res[2][1] - 0.0).abs()<e);
        assert!((res[2][2] - 1.00/2.00).abs()<e);

        let m = Matrix::new_from(2, 2, &[&[7.0, 10.0], &[15.0, 22.0]]).unwrap();
        let res = math::inv(&m).unwrap();
        let expected = Matrix::new_from(2, 2, &[&[5.5, -2.5], &[-3.75, 1.75]]).unwrap();
        assert!(res.equals(&expected));
    }

    #[test]
    fn compatible_determinado(){
        let m: Matrix = Matrix::new_from(3, 3, &[&[2.0, -1.0, 3.0], &[3.0, 6.0, 7.0], &[4.0, -2.0, 8.0]]).unwrap();
        let result: Matrix = Matrix ::new_from(3,1, &[&[1.0], &[2.0], &[3.0]]).unwrap();
        let res: Matrix = math :: data_loading(&m, &result);
        let c = math::solve_system(&res);
        assert!(c == math::Compatibility::CompatibleDeterminado);
    }

    #[test]
    fn incompatible_equation() {
        let new: Matrix = Matrix::new_from(2, 2, &[&[-2.0,-2.0], &[2.0,2.0]]).unwrap();
        let res: Matrix = Matrix::new_from(2,1, &[&[-6.0], &[2.0]]).unwrap();
        let total: Matrix= math::data_loading(&new,&res);
        let result = math::solve_system(&total);
        assert!(result == math::Compatibility::Incompatible);
    }

    #[test]
    fn very_incompatible() {
        let m: Matrix= Matrix::new_from(3,4,&[&[1.0, 1.0, 1.0, 4.0], &[2.0, 2.0, 2.0, 8.0], &[3.0, 3.0, 3.0, 45.0]]).unwrap();
        let result = math::solve_system(&m);

        assert!(result == math::Compatibility::Incompatible);
    }   

    #[test]
    fn undetermined(){
        let m: Matrix = Matrix::new_from(2, 3, &[&[2.0,1.0,4.0], &[4.0,2.0,8.0]]).unwrap();
        let result = math::solve_system(&m);
        assert!(result.is_compatible_indeterminado());
    }
}
