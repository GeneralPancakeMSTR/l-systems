// use dol_systems::{Predecessor, Successor, Productions, DOLString}; 
use std::collections::HashMap; 
// use std::rc::Rc; 

// use dol_systems::{State, Instruction}; 

#[derive(Debug, Clone)]
struct State(String); 

trait Symbol {    
    fn representation(&self) -> String; 
    fn evaluate(&self, state: & State) -> State; 
    // fn produce(&self, constants: HashMap<String,f64>) -> Vec<Box<dyn Symbol>>;
    fn produce(&self, constants: &Option<HashMap<&str,f64>>) -> Vec<Box<dyn Symbol>>; 
    // fn produce(&self, constants: &Box<dyn Constants>) -> Vec<Box<dyn Symbol>>; 
}

struct A {
    parameters: Vec<f64>
}

impl Symbol for A {
    fn representation(&self) -> String {
        String::from(format!("A({:?})",self.parameters))
    }

    fn evaluate(&self, state: & State) -> State {
        State(format!("A({} x {:?})",state.0, self.parameters))
    }

    fn produce(&self, constants: &Option<HashMap<&str, f64>>) -> Vec<Box<dyn Symbol>> {
        let s = self.parameters[0]; 

        let (r,p) = (1.456,1.414); // Defaults 

        let (r,p) = match constants {
            Some(hashmap) => {
                let r = match hashmap.get("r") {
                    Some(r) => *r, 
                    None => r,  
                };

                let p = match hashmap.get("p") {
                    Some(p) => *p, 
                    None => p, 
                }; 

                (r,p)
                
            },
            None => (r,p) // Defaults 
        };

        vec![Box::new(F{parameters:vec![s]}),Box::new(A{parameters:vec![s/r]}),Box::new(A{parameters:vec![p]})]
    }
}

struct F {
    parameters: Vec<f64>
}

impl Symbol for F {
    fn representation(&self) -> String {
        String::from(format!("F({:?})",self.parameters))
    }

    fn evaluate(&self, state: & State) -> State {
        State(format!("F({} x {:?})", state.0, self.parameters))
    }

    fn produce(&self, _constants: &Option<HashMap<&str,f64>>) -> Vec<Box<dyn Symbol>> {        
        vec![Box::new(F{parameters: self.parameters.clone()})]
    }        
}

fn main() {    
    let mut constants = HashMap::new(); 
    constants.insert("r",1.456); 
    constants.insert("p",1.414); 

    let pred_a = A{parameters: vec![1.0]};    
    // let axiom = LString(vec![pred_a]); // Something like this 
    let mut lstring: Vec<Box<dyn Symbol>> = vec![Box::new(pred_a)];

    //////////////// Axiom ////////////////
    for dynamic_symbol in lstring.iter() {
        println!("{}",dynamic_symbol.representation()); 
    }

    println!();

    //////////////// Production 0 ////////////////
    let mut lstring_pi: Vec<Box<dyn Symbol>> = Vec::new(); 
    
    for dynamic_symbol in lstring.iter() {
        let production = dynamic_symbol.produce(&None); 
        for produced_symbol in production {
            lstring_pi.push(produced_symbol);
        }
    }

    lstring = lstring_pi; 

    for dynamic_symbol in lstring.iter() {
        println!("{}",dynamic_symbol.representation()); 
    }

    println!();

    //////////////// Production 1 ////////////////
    let mut lstring_pi: Vec<Box<dyn Symbol>> = Vec::new(); 
    
    for dynamic_symbol in lstring.iter() {
        let production = dynamic_symbol.produce(&None); 
        for produced_symbol in production {
            lstring_pi.push(produced_symbol);
        }
    }

    lstring = lstring_pi; 

    for dynamic_symbol in lstring.iter() {
        println!("{}",dynamic_symbol.representation()); 
    }

    println!();

    //////////////// Evaluate ////////////////
    let mut current_state = State(String::from("identity"));     

    let mut states = vec![current_state.clone()]; 

    for dynamic_symbol in lstring.iter() {
        current_state = dynamic_symbol.evaluate(&current_state);         
        states.push(current_state.clone()); 
    }

    for state in states.iter() {
        println!("{:?}",state);
    }
}
