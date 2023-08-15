use std::f64::consts::PI; 

use matrix4::matrix4::Matrix4; 
use matrix4::vector::Vector3; 
use matrix4::angle::Radians;



fn main() {
    let matrix4_a = Matrix4::identity(); 
    let matrix4_b = Matrix4::identity(); 

    println!("{matrix4_a}");
    println!();

    println!("{matrix4_b}");
    println!();

    let matrix4_t = Matrix4::translation(Vector3::from([1,2,3]));
    println!("{matrix4_t}");
    println!();

    let matrix4_s = Matrix4::scaling(Vector3::from([1,2,3]));
    println!("{matrix4_s}");
    println!();

    let theta = Radians(PI/4.0);    

    let matrix4_rx = Matrix4::rotation_x(theta);
    println!("{matrix4_rx}");
    println!();

    let matrix4_ry = Matrix4::rotation_y(theta);
    println!("{matrix4_ry}");
    println!();

    let matrix4_rz = Matrix4::rotation_z(theta);
    println!("{matrix4_rz}");
    println!();

}

    





