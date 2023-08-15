// If this gets too tiresome: 
// https://nalgebra.org/

use std::fmt;
use std::ops::{
    Add,
    Sub    
};

pub trait Shape {
    fn shape(&self) -> ArrayDimensions; 
}
pub struct ArrayDimensions {
    pub rows: usize, 
    pub cols: usize
}

impl fmt::Display for ArrayDimensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"rows {} cols {}",self.rows,self.cols)
    }
}

pub struct ShapeError {
    cols_a: usize, 
    rows_b: usize
}

impl fmt::Display for ShapeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Dimensions incompatible: cols A ({}) != rows B ({})",self.cols_a,self.rows_b)
    }
}


//////////////// Row4 (Not used) ////////////////
// // row4: [ [f64, f64, f64, f64] ]
// pub struct Row4(
//     [[f64; 4]; 1]
// );

// impl fmt::Display for Row4 {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f,"[{}, {}, {}, {}]",self.0[0][0],self.0[0][1],self.0[0][2],self.0[0][3])
//     }
// }

// impl Row4 {
//     pub fn new() -> Row4 {
//         Row4([[0.0,0.0,0.0,0.0]])
//     }
// }

//////////////// Col4 (Not used) ////////////////
// // col4: [
// //     [f64],
// //     [f64],
// //     [f64],
// //     [f64]
// // ]
// pub struct Col4(
//     [[f64;1]; 4]
// );

// impl fmt::Display for Col4 {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f,"[{}]\n[{}]\n[{}]\n[{}]",self.0[0][0],self.0[1][0],self.0[2][0],self.0[3][0])
//     }
// }

// impl Col4 {
//     pub fn new() -> Col4 {
//         Col4([[0.0],[0.0],[0.0],[0.0]])
//     }
// }

// You need to have Copy and Clone traits for ops (+,-,*) to work "the way you expect" 
// I.e. not moving the operands into the operation (thus consuming them) 
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

// Not (significantly) different from deriving the Copy and Clone traits from macros 
// impl Copy for Matrix4 {}

// impl Clone for Matrix4 {
//     fn clone(&self) -> Matrix4 {
//         *self
//     }
// }

// pub struct<T> Matrix<T>(
//     pub T
// );

impl fmt::Display for Matrix4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let row0 = format!("[{}, {}, {}, {}]",self.0[0][0],self.0[0][1],self.0[0][2],self.0[0][3]);
        let row1 = format!("[{}, {}, {}, {}]",self.0[1][0],self.0[1][1],self.0[1][2],self.0[1][3]);
        let row2 = format!("[{}, {}, {}, {}]",self.0[2][0],self.0[2][1],self.0[2][2],self.0[2][3]);
        let row3 = format!("[{}, {}, {}, {}]",self.0[3][0],self.0[3][1],self.0[3][2],self.0[3][3]);

        write!(f,"{row0}\n{row1}\n{row2}\n{row3}")
    }
}

impl Matrix4 {
    pub fn from<T: Into<f64> + Copy>( arr: [[T; 4]; 4] ) -> Matrix4 {
        let mut a = Matrix4([[0.0; 4]; 4]);
        // Identically:
        // Matrix4([
        //     [0.0,0.0,0.0,0.0],
        //     [0.0,0.0,0.0,0.0],
        //     [0.0,0.0,0.0,0.0],
        //     [0.0,0.0,0.0,0.0]
        // ])         

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

    // pub fn pre_multiply(self, other:Matrix4) -> Matrix4 { }

    pub fn post_multiply<T: Shape>(self, other:T) -> Result<Box<dyn Shape>, ShapeError> {
        let k = if self.shape().cols == other.shape().rows {
            self.shape().cols
        } else {
            return Err( ShapeError { cols_a:self.shape().cols, rows_b: other.shape().rows } )
        };

        println!("{k}");



        Ok(Box::new(self))


    }

    // impl<T> Mul<T> for Matrix4 {
    //     type Output = Matrix4; 
    
    //     fn mul(self, other:T) -> Matrix4 {
    //         match other {
    //             Matrix4 => Matrix4
    //             _ => Matrix4::new()
    //         }
    //     }
    // }
    
    // [ ] pub fn TranslationMatrix -> Matrix4 {}
    // [ ] pub fn RotationMatrix -> Matrix4 {}
    // [ ] pub fn ScaleMatrix -> Matrix4 {}

}

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

impl Shape for Matrix4 {
    fn shape(&self) -> ArrayDimensions { 
        let rows = self.0.len(); 
        let cols = self.0[0].len(); 

        ArrayDimensions {rows, cols} 

    }
}

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
}

