use dol_systems::{Predecessor, Successor, Productions, DOLString}; 
use std::collections::HashMap; 
use std::rc::Rc; 

fn main() {
    // G = <V, w, P> 
    // DOL-System = <Alphabet, Axiom, Productions> 
    

    let a_r = Rc::new(Predecessor(String::from("a_r"))); 
    let a_l = Rc::new(Predecessor(String::from("a_l"))); 
    let b_r = Rc::new(Predecessor(String::from("b_r"))); 
    let b_l = Rc::new(Predecessor(String::from("b_l"))); 

    let mut productions = Productions(HashMap::new()); 
    productions.0.insert(Rc::clone(&a_r),Successor(vec![Rc::clone(&a_l),Rc::clone(&b_r)]));
    productions.0.insert(Rc::clone(&a_l),Successor(vec![Rc::clone(&b_l),Rc::clone(&a_r)]));
    productions.0.insert(Rc::clone(&b_r),Successor(vec![Rc::clone(&a_r)]));
    productions.0.insert(Rc::clone(&b_l),Successor(vec![Rc::clone(&a_l)]));

    // let mut axiom = DOLString(vec![Rc::clone(&a_r)]);
    let mut axiom = vec![Rc::clone(&a_r)];

    for n in 0..4 {
        println!("{n} {:?}",axiom);
        axiom = productions.evaluate(&axiom);
    };  

    println!("{:?}",axiom);

    let dolstring = DOLString(axiom); 

    println!("{dolstring}");

}
