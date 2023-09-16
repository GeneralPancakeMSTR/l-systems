use std::fmt; 

use crate::state::State; 
use crate::state::EvalReturns; 


//////////////// LSystem - Generics? ////////////////
pub trait ConstantsTrait: Clone {}

pub trait AlphabetTrait<C: ConstantsTrait>: Clone + fmt::Display {
    fn produce(&self, constants: &C) -> Vec<Self>; 
    fn evaluate(&self, state: &State) -> EvalReturns;
}

pub struct GLSystem<C: ConstantsTrait, S: AlphabetTrait<C>> {
    pub constants: C, 
    pub axiom: Vec<S> 
}

impl<C: ConstantsTrait, S: AlphabetTrait<C>> Clone for GLSystem<C,S> {
    fn clone(&self) -> GLSystem<C,S> {
        let constants = self.constants.clone(); 
        
        let mut axiom = Vec::new(); 

        for symbol in self.axiom.iter() {
            axiom.push(symbol.clone());
        };

        GLSystem{ constants: constants, axiom: axiom }
                
    }
}

impl<C: ConstantsTrait, S: AlphabetTrait<C>> fmt::Display for GLSystem<C,S>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new(); 
        for symbol in self.axiom.iter() {
            string.push_str(&format!("{symbol}"));
        }
        write!(f,"{string}")
    }
}

impl <C: ConstantsTrait, S: AlphabetTrait<C>> GLSystem<C,S> {
    pub fn produce(&self, iterations: u32) -> GLSystem<C,S> {        
        // Copy the axiom 
        let mut axiom: Vec<S> = Vec::new(); 
        
        for symbol in self.axiom.iter() {
            axiom.push(symbol.clone());
        };

        for _i in 0..iterations {
            let mut production: Vec<S> = Vec::new(); 

            for symbol in axiom.iter() {
                production.extend(symbol.produce(&self.constants));
            };

            axiom = production; 

        };

        GLSystem { constants: self.constants.clone(), axiom: axiom }

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



