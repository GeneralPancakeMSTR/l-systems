use std::fmt;
use crate::glsystem::ConstantsTrait; 
use crate::glsystem::AlphabetTrait;

use crate::state::State; 
use crate::state::EvalReturns; 
use crate::state::rx; 
use crate::state::tx; 

#[derive(Debug, Copy, Clone)]
pub struct Constants {
    pub r: f64, 
    pub p: f64
}

impl ConstantsTrait for Constants{}

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
            Pop => write!(f,"]")
        }
    }    
}

impl AlphabetTrait<Constants> for Alphabet {
    fn produce(&self, constants: &Constants) -> Vec<Alphabet> {
        let r = constants.r; 
        let p = constants.p; 

        match self {
            A{s} => {
                vec![F{x:s.clone()}, Push, A{s:s/r}, Pop, A{s:p}]
            },
            _ => {
                vec![self.clone()]
            }
        }
    }

    fn evaluate(&self, state: &State) -> EvalReturns {
        match self {
            A{s} => EvalReturns::State(rx(state,s.clone())), 
            F{x} => EvalReturns::State(tx(state,x.clone())), 
            Push => EvalReturns::PushState,
            Pop => EvalReturns::PopState
        }
    }
}