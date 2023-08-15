use crate::matrix4::Matrix4; 
use crate::vector::Vector3; 
use crate::angle::Radians; 

impl Matrix4 {
    pub fn translation(t: Vector3) -> Matrix4 {        
        let mut c = Matrix4::identity(); 

        c.0[0][3] = t.0[0]; 
        c.0[1][3] = t.0[1]; 
        c.0[2][3] = t.0[2]; 

        c
    }

    pub fn scaling(s: Vector3) -> Matrix4 {
        let mut c = Matrix4::identity();

        c.0[0][0] = s.0[0]; 
        c.0[1][1] = s.0[1]; 
        c.0[2][2] = s.0[2]; 

        c
    }

    pub fn rotation_x(theta: Radians) -> Matrix4 {
        let mut c = Matrix4::identity(); 

        c.0[1][1] = theta.0.cos(); 
        c.0[1][2] = theta.0.sin();
        c.0[2][1] = -theta.0.sin();
        c.0[2][2] = theta.0.cos();

        c 
    }

    pub fn rotation_y(theta: Radians) -> Matrix4 {
        let mut c = Matrix4::identity(); 

        c.0[0][0] = theta.0.cos(); 
        c.0[0][2] = -theta.0.sin();
        c.0[2][0] = theta.0.sin();
        c.0[2][2] = theta.0.cos();

        c 
    }

    pub fn rotation_z(theta: Radians) -> Matrix4 {
        let mut c = Matrix4::identity(); 

        c.0[0][0] = theta.0.cos(); 
        c.0[0][1] = -theta.0.sin();
        c.0[1][0] = theta.0.sin();
        c.0[1][1] = theta.0.cos();

        c 
    }
}

//////////////// Tests ////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn translation() {
        assert_eq!(Matrix4::translation(Vector3::from([1,2,3])),Matrix4::from([
            [1, 0, 0, 1],
            [0, 1, 0, 2],
            [0, 0, 1, 3],
            [0, 0, 0, 1]
        ]))
    }

    #[test]
    fn scaling() {
        assert_eq!(Matrix4::scaling(Vector3::from([1,2,3])),Matrix4::from([
            [1, 0, 0, 0],
            [0, 2, 0, 0],
            [0, 0, 3, 0],
            [0, 0, 0, 1]
        ]))
    }

    #[test]
    fn rotation_x() {
        assert_eq!(
            Matrix4::rotation_x(Radians(PI/4.0)),
            Matrix4::from([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, (PI/4.0).cos(), (PI/4.0).sin(), 0.0],
                [0.0, -(PI/4.0).sin(), (PI/4.0).cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ])
        )
    }

    #[test]
    fn rotation_y() {
        assert_eq!(
            Matrix4::rotation_y(Radians(PI/4.0)),
            Matrix4::from([
                [(PI/4.0).cos(), 0.0, -(PI/4.0).sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [(PI/4.0).sin(), 0.0, (PI/4.0).cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ])
        )
    }

    #[test]
    fn rotation_z() {
        assert_eq!(
            Matrix4::rotation_z(Radians(PI/4.0)),
            Matrix4::from([
                [(PI/4.0).cos(), -(PI/4.0).sin(), 0.0, 0.0],
                [(PI/4.0).sin(), (PI/4.0).cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ])
        )
    }    
}
