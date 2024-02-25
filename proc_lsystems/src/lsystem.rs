use std::fmt;

//////////////// State ////////////////
#[derive(Debug, Clone, PartialEq)]
pub struct State(pub String);

impl State {
    pub fn from(string: &str) -> Self {
        State(String::from(string))
    }
}

//////////////// EvalReturns ////////////////
// https://users.rust-lang.org/t/how-to-return-temporary-value-and-make-it-live-longer/24372/3
// I ended up not using a lifetime here, couldn't get it to work
// Or, at least, it seemed like more trouble than it was worth
#[derive(Debug)]
pub enum EvalReturns {
    State(State),
    DropState(State),
    PushState,
    PopState,
}

//////////////// Alphabet Implementation ////////////////
pub trait AlphabetTrait: Clone + fmt::Display {
    // This is a maybe, as in I'm not really sure we need this, but maybe?
    // type ConstantsList: Clone;
    // fn constants(&self) -> Self::ConstantsList;

    fn expand(&self) -> Vec<Self>;

    fn apply(&self, state: &State) -> EvalReturns;
}

//////////////// LSystem Implementation ////////////////
// I Guess you could make the State a generic
// Like, a State trait. But what would it implement?
// Also, not sure that's really necessary

pub struct LSystem<S: AlphabetTrait> {
    pub axiom: Vec<S>,
}

impl<S: AlphabetTrait> Clone for LSystem<S> {
    fn clone(&self) -> Self {
        let mut axiom = Vec::new();

        for symbol in self.axiom.iter() {
            axiom.push(symbol.clone());
        }

        LSystem { axiom }
    }
}

impl<S: AlphabetTrait> fmt::Display for LSystem<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();
        for symbol in self.axiom.iter() {
            string.push_str(&format!("{symbol}"));
        }
        write!(f, "{string}")
    }
}

impl<S: AlphabetTrait> LSystem<S> {
    pub fn produce_copy(&self, iterations: u32) -> LSystem<S> {
        let mut production: Vec<S> = Vec::new();

        match iterations {
            u32::MIN..=0 => self.clone(),
            _ => {
                for _i in 0..iterations {
                    for symbol in self.axiom.iter() {
                        production.extend(symbol.expand());
                    }
                }
                LSystem { axiom: production }
            }
        }
    }

    pub fn produce(&mut self, iterations: u32) -> () {
        let mut production: Vec<S> = Vec::new();

        match iterations {
            iterations if iterations <= 0 => (),
            _ => {
                for _i in 0..iterations {
                    for symbol in self.axiom.iter() {
                        production.extend(symbol.expand());
                    }
                }
                self.axiom = production
            }
        }
    }

    pub fn evaluate(&self, state: State) -> Vec<State> {
        // state: &State produces a lifetime problem
        let mut states = vec![state.clone()];
        let mut stack: Vec<State> = Vec::new();

        let mut current_state = state; // = state produces a lifetime problem (if you borrow the init state). Not sure how to resolve.

        for symbol in self.axiom.iter() {
            match symbol.apply(&current_state) {
                EvalReturns::State(state) => {
                    // Extract Evaluated State and Record it
                    // "Move forward and draw a line."
                    states.push(state.clone());
                    current_state = state;
                }
                EvalReturns::DropState(state) => {
                    // Extract Evaluated State but DO NOT Record it
                    // "Move forward without drawing a line."
                    current_state = state;
                }
                EvalReturns::PushState => {
                    // This comment is here for formatting purposes
                    stack.push(current_state.clone())
                }
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

//////////////// Tests ////////////////
#[cfg(test)]
mod tests {
    use super::*;

    //////////////// Implementation ////////////////
    #[derive(Debug, Copy, Clone, PartialEq)]
    enum Alphabet {
        A { s: f64 },
        F { x: f64 },
        DropF { x: f64 },
        Push,
        Pop,
    }

    use Alphabet::DropF;
    use Alphabet::Pop;
    use Alphabet::Push;
    use Alphabet::A;
    use Alphabet::F;

    impl fmt::Display for Alphabet {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                A { s } => write!(f, "A({:.3})", s),
                F { x } => write!(f, "F({:.3})", x),
                DropF { x } => write!(f, "f({:.3})", x),
                Push => write!(f, "["),
                Pop => write!(f, "]"),
            }
        }
    }

    impl AlphabetTrait for Alphabet {
        fn expand(&self) -> Vec<Self> {
            let r = 1.456;
            let p = 1.414;

            match self {
                A { s } => {
                    vec![
                        F { x: s.clone() },
                        Push,
                        A { s: s / r },
                        Pop,
                        DropF { x: s.clone() },
                        A { s: p },
                    ]
                }
                _ => vec![self.clone()],
            }
        }

        fn apply(&self, state: &State) -> EvalReturns {
            match self {
                A { s } => EvalReturns::State(rx(state, s.clone())),
                F { x } => EvalReturns::State(tx(state, x.clone())),
                DropF { x } => EvalReturns::DropState(tx(state, x.clone())),
                Push => EvalReturns::PushState,
                Pop => EvalReturns::PopState,
            }
        }
    }

    fn unwrap_eval(eval: EvalReturns) -> State {
        match eval {
            EvalReturns::State(state) => state,
            EvalReturns::DropState(state) => state,
            _ => State::from("Error: Tried to extract state from PushState or PopState"),
        }
    }

    fn rx(state: &State, angle_rads: f64) -> State {
        State(format!("Rotate ({}) by {} about x", state.0, angle_rads))
    }

    fn tx(state: &State, distance: f64) -> State {
        State(format!("Translate ({}) by {} along x", state.0, distance))
    }

    #[test]
    fn display_symbol() {
        // cargo test lsystem::tests::display_symbol -- --exact --show-output
        let a = A { s: 1.0 };
        let f = F { x: 1.0 };
        let df = DropF { x: (1.0) };
        let push = Push;
        let pop = Pop;

        println!("{a}");
        println!("{f}");
        println!("{df}");
        println!("{push}");
        println!("{pop}");

        assert_eq!(format!("{a}"), String::from("A(1.000)"));
        assert_eq!(format!("{f}"), String::from("F(1.000)"));
        assert_eq!(format!("{df}"), String::from("f(1.000)"));
        assert_eq!(format!("{push}"), String::from("["));
        assert_eq!(format!("{pop}"), String::from("]"));
    }

    #[test]
    fn expand_symbol() {
        // cargo test lsystem::tests::expand_symbol -- --exact --show-output

        let s = 1.0;

        let a = A { s };

        let production = a.expand();

        println!("{:?}", production);

        let r = 1.456;
        let p = 1.414;

        assert_eq!(
            production,
            vec![
                F { x: s.clone() },
                Push,
                A { s: s / r },
                Pop,
                DropF { x: s.clone() },
                A { s: p }
            ]
        );

        let x = 1.0;

        let f = F { x };

        let production = f.expand();

        println!("{:?}", production);

        assert_eq!(production, vec![F { x }]);
    }

    #[test]
    fn apply_symbol() {
        // cargo test lsystem::tests::apply -- --exact --show-output

        let s = 1.0;
        let a = A { s };

        let state0 = State::from("Init");

        let state1 = unwrap_eval(a.apply(&state0));

        println!("{:?}", state1);

        assert_eq!(state1, unwrap_eval(A { s }.apply(&state0)));
    }

    #[test]
    fn display_produce_display() {
        // cargo test lsystem::tests::display_produce_display -- --exact --show-output
        let r = 1.456;
        let p = 1.414;

        let s = 1.0;
        let mut lsystem = LSystem {
            axiom: vec![A { s }],
        };

        println!("{lsystem}");
        assert_eq!(format!("{lsystem}"), String::from("A(1.000)"));

        assert_eq!(lsystem.axiom, vec![A { s }]);

        lsystem.produce(1);

        println!("{lsystem}");

        assert_eq!(
            lsystem.axiom,
            vec![
                F { x: s.clone() },
                Push,
                A { s: s / r },
                Pop,
                DropF { x: s.clone() },
                A { s: p }
            ]
        );
    }

    #[test]
    fn produce_twice() {
        // cargo test lsystem::tests::produce_twice -- --exact --show-output
        let r = 1.456;
        let p = 1.414;

        let s = 1.0;
        let mut lsystem = LSystem {
            axiom: vec![A { s }],
        };

        // Production 0 (axiom)
        println!("{}", lsystem);
        assert_eq!(lsystem.axiom, vec![A { s }]);

        // Production 1
        lsystem.produce(1);
        println!("{}", lsystem);
        assert_eq!(
            lsystem.axiom,
            vec![
                F { x: s.clone() },
                Push,
                A { s: s / r },
                Pop,
                DropF { x: s.clone() },
                A { s: p }
            ]
        );

        // Production 2
        lsystem.produce(1);
        println!("{}", lsystem);
        assert_eq!(
            lsystem.axiom,
            vec![
                F { x: s.clone() },
                Push,
                F { x: s / r },
                Push,
                A { s: s / (r * r) },
                Pop,
                DropF { x: (s / r) },
                A { s: p },
                Pop,
                DropF { x: s },
                F { x: p },
                Push,
                A { s: p / r },
                Pop,
                DropF { x: p },
                A { s: p }
            ]
        );
    }

    #[test]
    fn evaluate() {
        // cargo test lsystem::tests::evaluate -- --exact --show-output
        let r = 1.456;
        let p = 1.414;

        let s = 1.0;
        let mut lsystem = LSystem {
            axiom: vec![A { s }],
        };

        // Production 0 (axiom)
        println!("{}", lsystem);
        assert_eq!(lsystem.axiom, vec![A { s }]);

        let state_init = State::from("Init");

        let prod_0 = lsystem.evaluate(state_init.clone());

        println!("{:?}", prod_0);

        assert_eq!(
            prod_0,
            vec![state_init.clone(), unwrap_eval(A { s }.apply(&state_init))]
        );

        // Production 1
        lsystem.produce(1);
        println!("{}", lsystem);

        let state_1 = unwrap_eval(F { x: s.clone() }.apply(&state_init));
        let state_2 = unwrap_eval(A { s: s / r }.apply(&state_1));
        let state_3 = unwrap_eval(DropF { x: s.clone() }.apply(&state_1));
        let state_4 = unwrap_eval(A { s: p.clone() }.apply(&state_3));

        let prod_1 = lsystem.evaluate(state_init.clone());
        println!("{:?}", prod_1);

        assert_eq!(prod_1, vec![state_init, state_1, state_2, state_4]);
    }
}
