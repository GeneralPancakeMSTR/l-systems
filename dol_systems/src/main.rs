use dol_systems::{Predecessor, Successor, Productions, DOLString}; 
use std::collections::HashMap; 
use std::rc::Rc; 

fn main() {
    // G = <V, w, P> 
    // DOL-System = <Alphabet, Axiom, Productions>     

    // This is now a unit test (anabaena_catenula_filament)

    let a_r = Rc::new(Predecessor(String::from("a_r"))); 
    let a_l = Rc::new(Predecessor(String::from("a_l"))); 
    let b_r = Rc::new(Predecessor(String::from("b_r"))); 
    let b_l = Rc::new(Predecessor(String::from("b_l"))); 

    let mut productions = Productions(HashMap::new()); 
    productions.0.insert(Rc::clone(&a_r),Successor(vec![Rc::clone(&a_l),Rc::clone(&b_r)]));
    productions.0.insert(Rc::clone(&a_l),Successor(vec![Rc::clone(&b_l),Rc::clone(&a_r)]));
    productions.0.insert(Rc::clone(&b_r),Successor(vec![Rc::clone(&a_r)]));
    productions.0.insert(Rc::clone(&b_l),Successor(vec![Rc::clone(&a_l)]));

    let mut axiom = DOLString(vec![Rc::clone(&a_r)]);

    println!("0 {}",axiom);

    for n in 1..5 {        
        // println!("{n} {:?}",axiom);        
        axiom = productions.evaluate(&axiom);
        println!("{n} {}",axiom);
    };  

    println!("{}",axiom);

}
