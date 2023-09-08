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

//////////////// Symbols Interface ////////////////
pub trait Symbol {
    fn representation(&self) -> String; 
    fn evaluate(&self, state: & State) -> State; 
    fn produce(&self, constants: Option<&HashMap<&str,f64>>) -> Vec<Box<dyn Symbol>>;
}

// Major Problem: Can't implement copy/clone 

//////////////// Enum ////////////////
pub struct Constants {
    pub r: f64, 
    pub p: f64
}

#[derive(Debug, Copy, Clone)]
pub enum Alphabet {
    A{s:f64},
    F{x:f64}
}

use Alphabet::A; 
use Alphabet::F; 

impl Alphabet {
    pub fn produce(&self, constants: &Constants) -> Vec<Alphabet> {
        let r = constants.r; 
        let p = constants.p; 

        match self {
            A{s} => {
                vec![F{x:1.0},A{s:s/r},A{s:p}]
            },
            // F{x} => {                
            //     vec![F{x: x.clone()}]
            // },
            _ => {
                vec![self.clone()]
            }
        }
    }
}

struct LSystem {
    constants: Constants, 
    axiom: Vec<Alphabet>
}

pub fn produce(axiom: &Vec<Alphabet>, constants: &Constants, iterations: u32) -> Vec<Alphabet> {
    println!("{iterations}");
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

//////////////// LString ////////////////
// ??? 

//////////////// LSystem ////////////////
// ??? 

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