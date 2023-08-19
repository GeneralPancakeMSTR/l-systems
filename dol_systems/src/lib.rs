// use std::collections::HashMap; 
// use std::rc::Rc; 
// use std::fmt; 


//////////////// Symbols/Commands ////////////////


// pub struct Instruction (
//     pub fn (state:State, parameters:Vec<f64>) -> (State)
// );


// pub fn Instruction 


//////////////// Tests ////////////////
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn anabaena_catenula_filament() {
//         let a_r = Rc::new(Predecessor(String::from("a_r"))); 
//         let a_l = Rc::new(Predecessor(String::from("a_l"))); 
//         let b_r = Rc::new(Predecessor(String::from("b_r"))); 
//         let b_l = Rc::new(Predecessor(String::from("b_l"))); 

//         let mut productions = Productions(HashMap::new()); 
//         productions.0.insert(Rc::clone(&a_r),Successor(vec![Rc::clone(&a_l),Rc::clone(&b_r)]));
//         productions.0.insert(Rc::clone(&a_l),Successor(vec![Rc::clone(&b_l),Rc::clone(&a_r)]));
//         productions.0.insert(Rc::clone(&b_r),Successor(vec![Rc::clone(&a_r)]));
//         productions.0.insert(Rc::clone(&b_l),Successor(vec![Rc::clone(&a_l)]));

//         let mut axiom = DOLString(vec![Rc::clone(&a_r)]);

//         for _n in 1..5 {            
//             axiom = productions.evaluate(&axiom); 
//         };       

//         assert_eq!(axiom.to_string(),"b_la_rb_la_ra_rb_la_ra_r")
        
//     }    
// }