use std::collections::HashMap; 
use std::rc::Rc; 
use std::fmt; 

//////////////// Symbol ////////////////
// [ ] ToDo 
// pub struct Symbol ... 

// pub struct Command {
//     pub description: String, 
//     pub symbol: String,     
//     pub instruction: Box<dyn Fn()> // Box<dyn Fn(state)>
// }





// pub instruction: fn(&Command) -> () // -> pub function: fn(&Command, state) -> state

//////////////// Predecessor ////////////////
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Predecessor(pub String); // -> pub struct Predecessor(pub Symbol);

impl fmt::Display for Predecessor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.0)
    }
}

//////////////// Successor ////////////////
#[derive(Debug)]
pub struct Successor(pub Vec<Rc<Predecessor>>);

impl Successor {
    fn expand(&self, sentence: &mut Vec<Rc<Predecessor>>) -> () {
        for predecessor in self.0.iter() {
            sentence.push(Rc::clone(&predecessor));
        }
    }
}

//////////////// (DOL) String ////////////////
pub struct DOLString(pub Vec<Rc<Predecessor>>);

impl fmt::Display for DOLString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut predecessor_symbols = String::new(); 
        for predecessor in &self.0 {
            let predecessor_symbol = format!("{}",predecessor); 
            predecessor_symbols.push_str(&predecessor_symbol);
        };
        
        write!(f,"{predecessor_symbols}")

    }
}

//////////////// System (Productions) ////////////////
pub struct Productions (
    pub HashMap<Rc<Predecessor>,Successor>
);

impl Productions {
    pub fn evaluate(&self, sentence: & Vec<Rc<Predecessor>>) -> Vec<Rc<Predecessor>> {
        let mut new_sentence: Vec<Rc<Predecessor>> = Vec::new(); 

        for predecessor in sentence {
            match self.0.get(predecessor.as_ref()) {
                Some(successor) => {
                    successor.expand(&mut new_sentence);              
                },
                None => new_sentence.push(Rc::clone(&predecessor)),
            }
        };
    
        new_sentence 

    }
}
