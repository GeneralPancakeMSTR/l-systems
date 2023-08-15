use std::fmt;
use std::ops::{
    Add,
    Sub,
    Mul
};

// If this gets too tiresome: 
// https://nalgebra.org/

///////////////// Matrix4 Struct ////////////////
// You need to have Copy and Clone traits for ops (+,-,*) to work "the way you expect" 
// I.e. so that operands are not consumed by the operation 
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Matrix4(
    pub [[f64; 4]; 4]
);
// [
//     [f64,f64,f64,f64],
//     [f64,f64,f64,f64],
//     [f64,f64,f64,f64],
//     [f64,f64,f64,f64]
// ]

//////////////// fmt::Display impl ////////////////
impl fmt::Display for Matrix4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let row0 = format!("[{}, {}, {}, {}]",self.0[0][0],self.0[0][1],self.0[0][2],self.0[0][3]);
        let row1 = format!("[{}, {}, {}, {}]",self.0[1][0],self.0[1][1],self.0[1][2],self.0[1][3]);
        let row2 = format!("[{}, {}, {}, {}]",self.0[2][0],self.0[2][1],self.0[2][2],self.0[2][3]);
        let row3 = format!("[{}, {}, {}, {}]",self.0[3][0],self.0[3][1],self.0[3][2],self.0[3][3]);

        write!(f,"{row0}\n{row1}\n{row2}\n{row3}")
    }
}

//////////////// impl Matrix4 ////////////////
impl Matrix4 {
    pub fn from<T: Into<f64> + Copy>( arr: [[T; 4]; 4] ) -> Matrix4 {
        let mut a = Matrix4([[0.0; 4]; 4]);

        for i in 0..=3 {
            for j in 0..=3 {
                a.0[i][j] = arr[i][j].into(); 
            }
        };
        a
    }

    pub fn new() -> Matrix4 {
        Matrix4::from([ 
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0]
        ])
    }

    pub fn identity() -> Matrix4 {
        Matrix4::from([
            [1, 0, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 1]
        ])
    }
}

///////////////// Matrix4 Ops ////////////////
impl Add<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn add(self, other:Matrix4) -> Matrix4 {
        let mut c = Matrix4::new(); 
        for i in 0..=3 {
            for j in 0..=3 {
                c.0[i][j] = self.0[i][j] + other.0[i][j]
            }
        }
        c
    }
}

impl Sub<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn sub(self, other:Matrix4) -> Matrix4 {
        let mut c = Matrix4::new(); 
        for i in 0..=3 {
            for j in 0..=3 {
                c.0[i][j] = self.0[i][j] - other.0[i][j]
            }
        }
        c
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4; 

    fn mul(self, other:Matrix4) -> Matrix4 {
        let mut c = Matrix4::new(); 
        for i in 0..=3 {
            for j in 0..=3 {                
                for k in 0..=3 {
                    c.0[i][j] += self.0[i][k]*other.0[k][j];
                };
            };
        };
        c
    }
}

///////////////// Tests ////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add(){
        let matrix_a = Matrix4::identity(); 
        let matrix_b = Matrix4::identity(); 
        let matrix_c = matrix_a + matrix_b;

        assert_eq!(matrix_c, Matrix4::from([
            [2, 0, 0, 0],
            [0, 2, 0, 0],
            [0, 0, 2, 0],
            [0, 0, 0, 2]
            ]));
    }

    #[test]
    fn sub(){
        let matrix_a = Matrix4::identity(); 
        let matrix_b = Matrix4::identity(); 
        let matrix_c = matrix_a - matrix_b;

        assert_eq!(matrix_c, Matrix4::from([
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0]
            ]));
    }

    #[test]
    fn mul_identity(){
        let matrix_a = Matrix4::identity(); 
        let matrix_b = Matrix4::identity(); 
        let matrix_c = matrix_a * matrix_b;

        assert_eq!(matrix_c, Matrix4::from([
            [1, 0, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 1]
            ]));
    }

    #[test]
    fn mul(){
        let matrix_a = Matrix4::from([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16]
        ]); 

        let matrix_b = Matrix4::from([
            [17, 18, 19, 20],
            [21, 22, 23, 24],
            [25, 26, 27, 28],
            [29, 30, 31, 32]
        ]); 
        
        let matrix_c = matrix_a * matrix_b;

        assert_eq!(matrix_c, Matrix4::from([
            [250, 260, 270, 280],
            [618, 644, 670, 696],
            [986, 1028, 1070, 1112],
            [1354, 1412, 1470, 1528]
            ]));
    }
}