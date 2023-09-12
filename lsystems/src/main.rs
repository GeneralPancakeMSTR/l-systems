use lsystems::EvalReturns;
use lsystems::State; 
use lsystems::Constants; 
use lsystems::Alphabet::{A,F}; 
use lsystems::{produce,write};

// skeletizzle recommends something like 
// enum Symbol {
//     A,
//     F,
//   }
//   impl Symbol {
//     fn operate_on_state(&self, state: State) {
//       match self {
//         Symbol::A => ...
//         ,,,
//       }
//     }
//   }

// Factory pattern? https://stackoverflow.com/questions/65847670/how-to-implement-abstract-factory-in-rust

fn main() { 
    let constants = Constants{r:1.456, p:1.414}; 
    let mut axiom = vec![A{s:1.0}]; 
    
    println!("{}",write(&axiom));
    println!(); 

    axiom = produce(&axiom, &constants, 1);

    println!("{}",write(&axiom));
    println!(); 

    let mut states: Vec<State> = vec![State::from(String::from("S0"))]; 
    let mut stack: Vec<State> = Vec::new(); 

    let mut current_state = states[0].clone(); 

    for symbol in axiom.iter() {
        match symbol.evaluate(&current_state) {
            EvalReturns::State(state) => {                
                states.push(state.clone());
                current_state = state;
            },
            EvalReturns::PushState => {
                stack.push(current_state.clone());                
            }, 
            EvalReturns::PopState => {                
                current_state = match stack.pop(){
                    Some(state) => state,
                    None => {                        
                        // Strictly speaking this should never happen,
                        // (it means a pop preceded a push),
                        // but in case it does, nothing has to be done.
                        // It just means "keep the current state."
                        current_state
                    }
                };
            }, 
        }
    }

    for (i,state) in states.iter().enumerate() {
        println!("S_{i} = {:?}",state)
    }
    

}
