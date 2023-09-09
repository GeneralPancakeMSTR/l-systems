use std::fmt; 
use std::collections::HashMap; 

//////////////// Example ////////////////
pub struct Something(i32);

impl Something {
    pub fn from(integer: i32) -> Something {
        Something(integer)
    }
}

//////////////// State ////////////////
#[derive(Debug, Clone)]
pub struct State(String); 

impl State {
    pub fn from(string: String) -> Self {
        State(string)
    }
}

//////////////// Symbols Interface ////////////////
pub trait Symbol {
    fn representation(&self) -> String; 
    fn evaluate(&self, state: & State) -> State; 
    fn produce(&self, constants: Option<&HashMap<&str,f64>>) -> Vec<Box<dyn Symbol>>;
}

// Major Problem: Can't implement copy/clone 

//////////////// Enum ////////////////
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

pub struct Constants {
    pub r: f64, 
    pub p: f64
}

#[derive(Debug, Copy, Clone)]
pub enum Alphabet {
    A{s:f64},
    F{x:f64}
}

impl fmt::Display for Alphabet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {        
        match self {
            A{s} => write!(f,"A({:.3})",s),
            F{x} => write!(f,"F({:.3})",x)
        }
    }
}

use Alphabet::A; 
use Alphabet::F; 

impl Alphabet {
    pub fn produce(&self, constants: &Constants) -> Vec<Alphabet> {
        let r = constants.r; 
        let p = constants.p; 

        match self {
            A{s} => {
                vec![F{x:*s},A{s:s/r},A{s:p}]
            },
            _ => {
                vec![self.clone()]
            }
        }
    }
}

impl Alphabet {
    pub fn evaluate(&self, state: &State) -> State {
        // Ultimately, we would like to be able to do something like 
        // self.op(state)         
        // or I guess 
        // self.op(s)(state) 
        // which would require a lambda function 
        // or ...
        match self {
            A{s} => rx(state,*s),
            F{x} => tx(state,*x)
        }
    }
}

// Could do... 
pub fn rx(state: &State, angle_rads: f64) -> State {
    State(format!("Rotate ({}) by {} about x",state.0,angle_rads))
}

pub fn tx(state: &State, l: f64) -> State {
    State(format!("Translate ({}) by {} along x",state.0,l))
}

//////////////// Functions that operate on a LSystem (as a Vec<Alphabet>) ////////////////
pub fn produce(axiom: &Vec<Alphabet>, constants: &Constants, iterations: u32) -> Vec<Alphabet> {    
    let mut production: Vec<Alphabet> = Vec::new(); 

    match iterations {
        0 => {            
            for symbol in axiom.iter() {
                production.push(symbol.clone()); 
            }
            production             
        }, 
        _ => {            
            for symbol in axiom.iter() {
                production.extend(symbol.produce(&constants));                
            };            
            return produce(&production, &constants, iterations-1); 
        }
    }
}

pub fn write(axiom: &Vec<Alphabet>) -> String {
    let mut string = String::new(); 
    for symbol in axiom.iter() {        
        string.push_str(&format!("{symbol}"));
    }
    string 
}



//////////////// LString ////////////////
// ??? 

//////////////// LSystem ////////////////
// struct LSystem {
//     constants: Constants, 
//     axiom: Vec<Alphabet>
// }

// impl LSystem {
//     display ... 
//     produce ... 
//     evaluate ... 
// }



//////////////// Symbols ////////////////
pub struct ASymbol {
    pub parameters: Vec<f64>
}

impl Symbol for ASymbol {
    fn representation(&self) -> String {
        String::from(format!("A({:?})",self.parameters))
    }

    fn evaluate(&self, state: & State) -> State {
        State(format!("A({} x {:?})",state.0, self.parameters))
    }

    fn produce(&self, constants: Option<&HashMap<&str,f64>>) -> Vec<Box<dyn Symbol>> {
        let s = self.parameters[0]; 

        let (r,p) = (1.0,3.0); // Defaults 

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

        vec![Box::new(FSymbol{parameters:vec![s]}),Box::new(ASymbol{parameters:vec![s/r]}),Box::new(ASymbol{parameters:vec![p]})]
    }
}

pub struct FSymbol {
    pub parameters: Vec<f64>
}

impl Symbol for FSymbol {
    fn representation(&self) -> String {
        String::from(format!("F({:?})",self.parameters))
    }

    fn evaluate(&self, state: & State) -> State {
        State(format!("F({} x {:?})", state.0, self.parameters))
    }

    fn produce(&self, _constants: Option<&HashMap<&str,f64>>) -> Vec<Box<dyn Symbol>> {        
        vec![Box::new(FSymbol{parameters: self.parameters.clone()})]
    }        
}