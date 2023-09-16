use std::fmt; 

use crate::state::State; 
use crate::state::rx; 
use crate::state::tx; 
use crate::glsystem::EvalReturns; 

//////////////// Enum ////////////////
// Credit to skeletizzle for recommending enum 

// struct Rx {
//     t:f64 // theta 
// }
// impl Rx {
//     fn evaluate(&self, state: &State) -> State {
//         State(format!("Rotate {} by {} about x",state.0, self.t))
//     }
// }
// A<Rx>{s:f64} ? 
// F<Tx>{x:f64} ? 
// Such that 
// pub enum Alphabet {
//     A<Rx>{s:f64}
//     F<Tx>{x:f64}
// }
// or maybe 
// A = Symbol::RotateX; 
// F = Symbol::TranslateX; 
// Could you do 
// pub enum Alphabet {
//     A{Symbol::RotateX}{s:f64}
//     F{Symbol::TranslateX}{s:f64}
// }

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
// impl ::from ? 

//////////////// LSystem ////////////////
// impl ::from ? 

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

    pub fn evaluate(&self, state: State) -> Vec<State> {
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