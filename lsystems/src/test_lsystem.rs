//////////////// Example of a Successful LSystem Implementation ////////////////
// Being the first successful LSystem proof-of-concept I managed, this is maintained for posterity. 
// It is kept for reference and demonstrative purposes, and deliberately encapsulated and independent of the rest of the code in this project. 

use std::fmt; 

//////////////// State ////////////////
#[derive(Debug, Clone)]
pub struct State(String); 

impl State {
    pub fn from(string: String) -> Self {
        State(string)
    }
}

pub fn rx(state: &State, angle_rads: f64) -> State {
    State(format!("Rotate ({}) by {} about x",state.0,angle_rads))
}

pub fn tx(state: &State, l: f64) -> State {
    State(format!("Translate ({}) by {} along x",state.0,l))
}

//////////////// EvalReturns ////////////////
pub enum EvalReturns {
    State(State),
    PushState, 
    PopState
}

//////////////// Enum ////////////////
#[derive(Debug, Copy, Clone)]
pub struct Constants {
    pub r: f64, 
    pub p: f64
}

#[derive(Debug, Copy, Clone)]
pub enum Alphabet {
    A{s:f64},
    F{x:f64},
    Push,
    Pop
}

use Alphabet::A; 
use Alphabet::F; 
use Alphabet::Push; 
use Alphabet::Pop; 

impl fmt::Display for Alphabet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {        
        match self {
            A{s} => write!(f,"A({:.3})",s),
            F{x} => write!(f,"F({:.3})",x),
            Push => write!(f,"["),
            Pop => write!(f,"]"),
        }
    }
}

impl Alphabet {
    pub fn produce(&self, constants: &Constants) -> Vec<Alphabet> {
        let r = constants.r; 
        let p = constants.p; 

        match self {
            A{s} => {
                vec![F{x:*s},Push,A{s:s/r},Pop,A{s:p}]
            },
            _ => {
                vec![self.clone()]
            }
        }
    }
}

impl Alphabet {
    pub fn evaluate(&self, state: &State) -> EvalReturns {
        // Ultimately, we would like to be able to do something like 
        // self.op(state)         
        // or I guess 
        // self.op(s)(state) 
        // which would require a lambda function 
        // or ...
        match self {
            A{s} => EvalReturns::State(rx(state,s.clone())),
            F{x} => EvalReturns::State(tx(state,x.clone())),
            Push => EvalReturns::PushState,
            Pop => EvalReturns::PopState,
        }
    }
}

//////////////// LSystem ////////////////
pub struct LSystem {
    pub constants: Constants, 
    pub axiom: Vec<Alphabet>
}

impl Clone for LSystem {
    fn clone(&self) -> Self {
        let constants = self.constants.clone(); 
        
        let mut axiom = Vec::new(); 
        
        for symbol in self.axiom.iter() {
            axiom.push(symbol.clone());
        };
        
        LSystem { constants: constants, axiom: axiom }
    }
}

impl fmt::Display for LSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new(); 
        for symbol in self.axiom.iter() {        
            string.push_str(&format!("{symbol}"));
        }
        write!(f,"{string}")
    }
}

impl LSystem {
    pub fn produce(&self, iterations: u32) -> LSystem {
        let mut axiom: Vec<Alphabet> = Vec::new(); 

        for symbol in self.axiom.iter() {
            axiom.push(symbol.clone());
        };

        for _i in 0..iterations {            
            let mut production: Vec<Alphabet> = Vec::new(); 

            for symbol in axiom.iter() {
                production.extend(symbol.produce(&self.constants));
            };

            axiom = production; 

        };

        LSystem{constants: self.constants.clone(), axiom: axiom}

    }

    pub fn evaluate(&self, state: &State) -> Vec<State> {
        let mut states: Vec<State> = vec![state.clone()];         
        let mut stack: Vec<State> = Vec::new(); 

        let mut current_state = state.clone(); 

        for symbol in self.axiom.iter() {            
            match symbol.evaluate(&current_state) { 
                EvalReturns::State(state) => {
                    states.push(state.clone()); 
                    current_state = state; 
                },
                EvalReturns::PushState => {
                    stack.push(current_state.clone()); 
                },
                EvalReturns::PopState => {
                    current_state = match stack.pop() {
                        Some(state) => state, 
                        None => {
                            // Strictly speaking this should never happen,
                            // as it means a pop preceded a push,
                            // which syntactically doesn't make sense. 
                            // But in case it does, nothing has to be done. 
                            // It amounts to "keep the current state."
                            current_state
                        }
                    }
                }
            }
        }

        states

    }
}

//////////////// Test/Demonstration ////////////////
pub fn test_lsystem() {
    println!();
    
    let constants = Constants{r: 1.456, p:1.414}; 

    let s0 = 1.0; 
    let axiom = vec![A{s:s0}]; 

    let mut lsystem = LSystem{constants: constants, axiom: axiom};    
    
    println!("Axiom: {lsystem}");

    lsystem = lsystem.produce(1); 

    println!("Production 1: {lsystem}");

    lsystem = lsystem.produce(1); 

    println!("Production 2: {lsystem}");

    let state0 = State::from(String::from("S0"));

    let states = lsystem.evaluate(&state0); 

    println!();

    for (i, state) in states.iter().enumerate() {
        println!("S{i} = {:?}",state);
    }

}