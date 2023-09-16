//////////////// State ////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct State(String); 

impl State {
    pub fn from(string: String) -> Self {
        State(string)
    }
}

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

// Could do... 
pub fn rx(state: &State, angle_rads: f64) -> State {
    State(format!("Rotate ({}) by {} about x",state.0,angle_rads))
}

pub fn tx(state: &State, l: f64) -> State {
    State(format!("Translate ({}) by {} along x",state.0,l))
}
