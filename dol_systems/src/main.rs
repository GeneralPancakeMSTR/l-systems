// use dol_systems::{Predecessor, Successor, Productions, DOLString}; 
// use std::collections::HashMap; 
// use std::rc::Rc; 

// use dol_systems::{State, Instruction}; 

#[derive(Debug, Clone)]
struct State(String); 

trait Symbol {
    fn representation(&self) -> String; 
    fn evaluate(&self, state: & State) -> State; 
    fn produce(&self,constants: & Vec<f64>) -> Vec<Box<dyn Symbol>>;
}

struct A {
    parameters: Vec<f64>
}

struct F {
    parameters: Vec<f64>
}

impl Symbol for A {
    fn representation(&self) -> String {
        String::from(format!("A({:?})",self.parameters))
    }

    fn evaluate(&self, state: & State) -> State {
        State(format!("A({} x {:?})",state.0,self.parameters))
    }

    fn produce(&self, constants: & Vec<f64>) -> Vec<Box<dyn Symbol>> {
        let s = self.parameters[0]; 
        let r = constants[0]; 

        vec![Box::new(F{parameters:vec![s]}),Box::new(A{parameters:vec![s/r]})]
    }
}

impl Symbol for F {
    fn representation(&self) -> String {
        String::from(format!("F({:?})",self.parameters))
    }

    fn evaluate(&self, state: & State) -> State {
        State(format!("F({} x {:?})",state.0,self.parameters))
    }

    fn produce(&self, _constants: & Vec<f64>) -> Vec<Box<dyn Symbol>> {
        let s = self.parameters[0];                 
        vec![Box::new(F{parameters:vec![s]})]
    }
}
fn main() {
    let pred_a = A{parameters: vec![1.0]};    

    let constants = vec![1.456]; 
    let lstring_axiom: Vec<Box<dyn Symbol>> = vec![Box::new(pred_a)]; 

    //////////////// Axiom ////////////////
    for dynamic_symbol in lstring_axiom.iter() {
        println!("{}",dynamic_symbol.representation());
    }

    println!();
    
    //////////////// Production 0 ////////////////
    let mut lstring_p0: Vec<Box<dyn Symbol>> = Vec::new(); 

    for dynamic_symbol in lstring_axiom.iter() {        
        let production = dynamic_symbol.produce(&constants); 

        for produced_symbol in production { 
            lstring_p0.push(produced_symbol);
        }        
    }    

    for dynamic_symbol in lstring_p0.iter() {
        println!("{}",dynamic_symbol.representation());        
    }

    println!();

    //////////////// Production 1 ////////////////
    let mut lstring_p1: Vec<Box<dyn Symbol>> = Vec::new(); 

    for dynamic_symbol in lstring_p0.iter() {
        let production = dynamic_symbol.produce(&constants); 

        for produced_symbol in production {
            lstring_p1.push(produced_symbol);
        }
    }

    for dynamic_symbol in lstring_p1.iter() {
        println!("{}",dynamic_symbol.representation());
    }

    println!();

    //////////////// Production 2 ////////////////
    let mut lstring_p2: Vec<Box<dyn Symbol>> = Vec::new(); 

    for dynamic_symbol in lstring_p1.iter() {
        let production = dynamic_symbol.produce(&constants); 

        for produced_symbol in production {
            lstring_p2.push(produced_symbol);
        }
    }

    for dynamic_symbol in lstring_p2.iter() {
        println!("{}",dynamic_symbol.representation());
    }

    println!();

    //////////////// Evaluate ////////////////        
    let mut current_state = State(String::from("An Initial State")); 

    let mut states = vec![current_state.clone()];

    for dynamic_symbol in lstring_p2.iter() {        
        current_state = dynamic_symbol.evaluate(&current_state);
        states.push(current_state.clone());        
    }

    for state in states.iter(){
        println!("{:?}",state);
    }

}
