mod math;
mod structs;
mod exp_interpreter;

fn main() {}

#[cfg(test)]
mod tests {
    use crate::{math, structs::Matrix};

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
    fn is_squared() {
        assert!(create2by2().is_squared());
        assert!(!Matrix::new_from (1, 2, &[&[1.0, 2.0]]).unwrap().is_squared())
    }

    #[test]
    fn equals() {
        let m1 = create2by2();
        let m2 = create2by2();
        assert!(m1.equals(m2));
        let m3 = Matrix::new_from (2, 3, &[&[1.0, 2.0, 3.0], &[3.0, 4.0, 3.0]]).unwrap();
        assert!(!m1.equals(m3));
    }

    #[test] //this test should return when the matrix sum is not possible
    fn matrix_sum_err() {
        let m1 = Matrix::new_from (2, 3, &[&[1.0, 2.0, 3.0], &[3.0, 4.0, 3.0]]).unwrap();
        let m2 = create2by2();
        math::sum(&m1, &m2).unwrap_err(); // fail 
    }

    #[test]  //this test should return that the matrix sum is correct
    fn matrix_sum_ok() {
        let m1= create2by2();
        let m2= create2by2();
        let res= math::sum(&m1, &m2).unwrap();
        
        assert_eq!(res[0][0], 2.0);
        assert_eq!(res[0][1], 4.0);
        assert_eq!(res[1][0], 6.0);
        assert_eq!(res[1][1], 8.0);
    }

    #[test]
    fn determinant() {
        let m = create2by2();
        assert_eq!(math::det(&m).unwrap(), -2.0);
    }

    #[test]
    fn orthogonal_test() {
        let m= Matrix::new_from(2,2, &[&[1.0,-1.0], &[1.0,1.0]]).unwrap();
        let res= math::inverse_ortogonal_matrix( &m ).unwrap();
        
        assert_eq!(res[0][0], 1.0);
        assert_eq!(res[0][1], 1.0);
        assert_eq!(res[1][0], -1.0);
        assert_eq!(res[1][1], 1.0);
    }
        #[test] // test for id matrix
    fn test_id() {
        let n:usize=3;
        let res: Matrix= math::id_matrix(n).unwrap();
        
        assert_eq!(res[0][0],1.0);
        assert_eq!(res[1][1],1.0);
        assert_eq!(res[2][2],1.0);
    }
    #[test] //test for trasp matrix 
    fn test_trasp() {
        let m: Matrix= Matrix::new_from(2,2, &[&[2.0,-1.0], &[3.0,6.0]]).unwrap();
        let res: Matrix= math::trasp_squared_matrix(&m).unwrap();
         
        assert_eq!(res[0][0], 2.0);
        assert_eq!(res[0][1], 3.0);
        assert_eq!(res[1][0], -1.0);
        assert_eq!(res[1][1], 6.0);
    }
}
