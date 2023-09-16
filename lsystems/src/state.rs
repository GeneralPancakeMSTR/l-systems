//////////////// State ////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct State(String); 

impl State {
    pub fn from(string: String) -> Self {
        State(string)
    }
}

// Could do... 
pub fn rx(state: &State, angle_rads: f64) -> State {
    State(format!("Rotate ({}) by {} about x",state.0,angle_rads))
}

pub fn tx(state: &State, l: f64) -> State {
    State(format!("Translate ({}) by {} along x",state.0,l))
}
