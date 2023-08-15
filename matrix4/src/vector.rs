// use std::fmt;
// use std::ops::{
//     Add,
//     Sub,
//     Mul
// };

//////////////// Vector3 Struct ////////////////
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector3(
    pub [f64; 3]
);

// [] impl fmt::Display for Vector3 

//////////////// impl Vector3 ////////////////
impl Vector3 {
    pub fn from<T: Into<f64> + Copy>( arr: [T; 3] ) -> Vector3 {
        let mut v = Vector3([0.0; 3]);

        v.0[0] = arr[0].into(); 
        v.0[1] = arr[1].into(); 
        v.0[2] = arr[2].into(); 
        
        v
    }

    pub fn new() -> Vector3 {
        Vector3::from([0,0,0])
    }

}

//////////////// Tests ////////////////
#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn from(){
        assert_eq!(Vector3::from([0,1,2]),Vector3([0.0,1.0,2.0]))
    }

    #[test]
    fn new(){
        assert_eq!(Vector3::new(),Vector3([0.0,0.0,0.0]))
    }
}