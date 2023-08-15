use std::f64::consts::PI; 

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Radians (
    pub f64
);

impl Radians {
    pub fn degrees(&self) -> f64 {
        self.0 * 180.0/PI
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Degrees (
    pub f64
);

impl Degrees {
    pub fn radians(&self) -> f64 {
        self.0 * PI/180.0
    }
}

//////////////// Tests ////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radians_to_degrees(){
        let angle_radians = Radians(PI);
        assert_eq!(angle_radians.degrees(),180.0)
    }

    #[test]
    fn degrees_to_radians(){
        let angle_degrees = Degrees(180.0);
        assert_eq!(angle_degrees.radians(),PI)
    }
}

//////////////// Cooler but more complicated approach ////////////////
// Also type bounds don't work the way I want them to 

// e.g. you can do like 
// fn radians_fn<T:Radians>(rads:T) {T ... }
// but then with e.g. 
// let angle = <Angle as Radians>::from(PI);
// you have to call it like 
// radians_fn<Angle>(angle)
// Which is annoying, I want the call to be 
// radians_fn(angle) // Breaks if Angle doesn't implement Radians. 

// #[derive(Copy, Clone, PartialEq, Debug)]
// pub struct Angle (
//     pub f64
// );

// #[derive(Copy, Clone, PartialEq, Debug)]
// pub struct Angle (
//     pub f64
// );

// pub trait Radians {
//     fn from(radians: f64) -> Angle;
//     fn degrees(&self) -> f64; 
// }

// impl Radians for Angle {
//     fn from(radians: f64) -> Angle {
//         Angle(radians)
//     }

//     fn degrees(&self) -> f64 {
//         self.0 * 180.0/PI
//     }
// }

// pub trait Degrees {
//     fn from(degrees: f64) -> Angle; 
//     fn radians(&self) -> f64; 
// }

// impl Degrees for Angle {
//     fn from(degrees: f64) -> Angle {
//         Angle(degrees)
//     }

//     fn radians(&self) -> f64 {
//         self.0 * PI/180.0
//     }
// }

//////////////// Tests ////////////////
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn radians_to_degrees(){
//         let angle_radians = <Angle as Radians>::from(PI);
//         assert_eq!(angle_radians.degrees(),180.0)
//     }

//     #[test]
//     fn degrees_to_radians(){
//         let angle_degrees = <Angle as Degrees>::from(180.0);
//         assert_eq!(angle_degrees.radians(),PI)
//     }
// }









