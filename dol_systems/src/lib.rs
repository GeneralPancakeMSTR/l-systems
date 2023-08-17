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
    fn expand(&self, dol_string: &mut DOLString) -> () {
        for predecessor in self.0.iter() {
            dol_string.push(Rc::clone(predecessor));            
        }
    }
}

//////////////// (DOL) String ////////////////
#[derive(Debug)]
pub struct DOLString(pub Vec<Rc<Predecessor>>);

impl fmt::Display for DOLString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {        
        write!(f,"{}",&self.to_string())
    }
}

impl DOLString {
    pub fn push(&mut self, predecessor: Rc<Predecessor>) -> () {
        self.0.push(predecessor); 
    }

    pub fn to_string(&self) -> String {
        let mut predecessor_symbols = String::new(); 
        for predecessor in &self.0 {
            let predecessor_symbol = format!("{predecessor}");
            predecessor_symbols.push_str(&predecessor_symbol);
        };

        predecessor_symbols

    }
}

//////////////// System (Productions) ////////////////
pub struct Productions (
    pub HashMap<Rc<Predecessor>,Successor>
);

impl Productions {
    pub fn evaluate(&self, dol_string: & DOLString) -> DOLString {
        let mut new_dol_string = DOLString(Vec::new()); 

        for predecessor in &dol_string.0 {
            match self.0.get(predecessor.as_ref()) {
                Some(successor) => {
                    successor.expand(&mut new_dol_string);
                },
                None => new_dol_string.push(Rc::clone(&predecessor)),
            }
        };    

        new_dol_string 

    }
}

//////////////// Tests ////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anabaena_catenula_filament() {
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

        for _n in 1..5 {            
            axiom = productions.evaluate(&axiom); 
        };       

        assert_eq!(axiom.to_string(),"b_la_rb_la_ra_rb_la_ra_r")
        
    }    
}