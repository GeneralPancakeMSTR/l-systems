pub mod affine;

pub mod vector;

pub mod matrix4; 

pub mod angle; 

///////////////// Alternative that seems complicated ////////////////
// trait MyTrait {
//     type MySubtype;
// }

// struct MyType {}

// impl MyTrait for MyType {
//     type MySubtype = Subtype;
// }

// struct Subtype {}

// impl Subtype {
//     pub fn bar() {
//         println!("it's bar");
//     }
// }

// <MyType as MyTrait>::MySubtype::bar()