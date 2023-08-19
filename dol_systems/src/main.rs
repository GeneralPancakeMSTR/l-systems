// use dol_systems::{Predecessor, Successor, Productions, DOLString}; 
// use std::collections::HashMap; 
// use std::rc::Rc; 

// use dol_systems::{State, Instruction}; 

#[derive(Debug)]
struct State(String); 

struct Instruction(
    Box<dyn Fn(& State, Vec<f64>) -> State>
);

impl Instruction {
    fn evaluate(&self, state: & State, parameters: Vec<f64> ) -> State {
        self.0(&state, parameters)
    }
}

struct Symbol {
    representation: String,
    command: Instruction
}



fn main() {
    // G = <V, w, P> 
    // DOL-System = <Alphabet, Axiom, Productions>

    // let a = Predecessor("A",instruction);

    // let predecessor = Symbol{repr: String::from("A"),command: String::from("Move forward by 1")}; 
    // println!("{}",predecessor.repr); 
    // println!("{}",predecessor.command); 


    // Symbol.instruction.evaluate(&state, parameters); 
    // let command = Command(Box::new(actual_instruction));

    let s = State(String::from("An Initial State"));
    let p = vec![0.0,1.0,2.0]; 

    let predecessor_a = Symbol{representation:String::from("A"),command:Instruction(Box::new(actual_instruction))};

    println!("Predecessor symbol: {}",predecessor_a.representation);
    let new_state = predecessor_a.command.evaluate(&s, p); 
    println!("{:?}",new_state);

    let p = vec![3.0,4.0,5.0]; 
    let new_state = predecessor_a.command.evaluate(&new_state, p); 
    println!("{:?}",new_state);

}

fn actual_instruction(state: & State, parameters: Vec<f64>) -> State {
    State(format!("{:?} x {}",parameters,state.0))
}

// You could imagine doing something like this too 
// A command/instruction constructor 
// fn rotate_by(axis: Vector, angle: Radians) -> Box<dyn Fn(& State, Vec<f64>) -> State> { }

// For reference 
// fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
//     Box::new(|x| x + 1)
// }
                                

