use std::collections::HashMap; 
use std::rc::Rc; 
// use std::fmt;

fn main() {
    // G = <V, w, P> 
    // DOL-System = <Alphabet, Axiom, Productions> 
    
    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Predecessor(String);

    let a_r = Rc::new(Predecessor(String::from("a_r"))); 
    let a_l = Rc::new(Predecessor(String::from("a_l"))); 
    let b_r = Rc::new(Predecessor(String::from("b_r"))); 
    let b_l = Rc::new(Predecessor(String::from("b_l"))); 

    #[derive(Debug)]
    struct Successor(Vec<Rc<Predecessor>>);

    impl Successor {
        fn expand(&self, sentence: &mut Vec<Rc<Predecessor>>) -> () {
            for predecessor in self.0.iter() {
                sentence.push(Rc::clone(&predecessor));
            }
        }
    }

    let mut productions: HashMap<Rc<Predecessor>,Successor> = HashMap::new(); 
    productions.insert(Rc::clone(&a_r),Successor(vec![Rc::clone(&a_l),Rc::clone(&b_r)]));
    productions.insert(Rc::clone(&a_l),Successor(vec![Rc::clone(&b_l),Rc::clone(&a_r)]));
    productions.insert(Rc::clone(&b_r),Successor(vec![Rc::clone(&a_r)]));
    productions.insert(Rc::clone(&b_l),Successor(vec![Rc::clone(&a_l)]));

    println!("{:?}",productions);
    
    
    // println!("{axiom}");    
    // -> vec![&a_l, &b_r]
    // -> vec![&b_l, &a_r, &a_r]
    // -> vec![&a_l, &a_l, &b_r, &a_l, &b_r]

    let mut axiom = vec![Rc::clone(&a_r)];

    //////////////// Loop 1 (n=0) ////////////////   
    let mut temp_axiom: Vec<Rc<Predecessor>> = Vec::new(); 

    for predecessor in axiom {        
        match productions.get(predecessor.as_ref()) {
            Some(successor) => {successor.expand(&mut temp_axiom)}
            None => temp_axiom.push(Rc::clone(&predecessor))
        }
    }

    axiom = temp_axiom; 
    println!("{:?}",axiom);
    

}



