use std::fmt;

//////////////// EvalReturns ////////////////
use crate::state::State;

#[derive(Debug, PartialEq)]
pub enum EvalReturns {
    State(State),
    DropState(State),
    PushState,
    PopState,
}

impl EvalReturns {
    // Dunno why complaining about this not being used, it's used in the unit tests
    fn unwrap_state(&self) -> State {
        match self {
            EvalReturns::State(state) => state.clone(), // #ToDo actually move state out of EvalReturns enum
            EvalReturns::DropState(state) => state.clone(),
            EvalReturns::PushState => {
                State::from(String::from("Error: Cannot extract state from PushState."))
            } // Improvements?
            EvalReturns::PopState => {
                State::from(String::from("Error: Cannot extract state from PopState."))
            } // Improvements?
        }
    }
}

//////////////// Canned Symbols ? ////////////////
// struct Push;
// struct Pop;

//////////////// LSystem - Generics ////////////////
pub trait ConstantsTrait: Clone {}

pub trait AlphabetTrait<C: ConstantsTrait>: Clone + fmt::Display {
    fn produce(&self, constants: &C) -> Vec<Self>;
    fn evaluate(&self, state: &State) -> EvalReturns;
}

// Interesting thought - maybe LSystem doesn't need to know about the constants?
// Seems like the Alphabet is all that really needs them?
// "LSystem" could maybe even just be "Axiom"?
// produce needs the constants...
// Would it be better to make these functions,
// as opposed to implementations on (the LSystem) struct?
// Something like
// Produce(Axiom,2);
// Evaluate(Axiom,Constants,State);
// Actually I don't think that works either, cause you need things concretely defined before hand.
// However, you could use a macro to expand those functions out, but, is that any better than what we've got?

pub struct LSystem<C: ConstantsTrait, S: AlphabetTrait<C>> {
    pub constants: C,
    pub axiom: Vec<S>,
}

impl<C: ConstantsTrait, S: AlphabetTrait<C>> Clone for LSystem<C, S> {
    fn clone(&self) -> LSystem<C, S> {
        let constants = self.constants.clone();

        let mut axiom = Vec::new();

        for symbol in self.axiom.iter() {
            axiom.push(symbol.clone());
        }

        LSystem {
            constants: constants,
            axiom: axiom,
        }
    }
}

impl<C: ConstantsTrait, S: AlphabetTrait<C>> fmt::Display for LSystem<C, S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();
        for symbol in self.axiom.iter() {
            string.push_str(&format!("{symbol}"));
        }
        write!(f, "{string}")
    }
}

impl<C: ConstantsTrait, S: AlphabetTrait<C>> LSystem<C, S> {
    pub fn produce(&self, iterations: u32) -> LSystem<C, S> {
        // Copy the axiom
        let mut axiom: Vec<S> = Vec::new();

        for symbol in self.axiom.iter() {
            axiom.push(symbol.clone());
        }

        for _i in 0..iterations {
            let mut production: Vec<S> = Vec::new();

            for symbol in axiom.iter() {
                production.extend(symbol.produce(&self.constants));
            }

            axiom = production;
        }

        LSystem {
            constants: self.constants.clone(),
            axiom: axiom,
        }
    }

    pub fn evaluate(&self, state: &State) -> Vec<State> {
        let mut states: Vec<State> = vec![state.clone()];
        let mut stack: Vec<State> = Vec::new();

        let mut current_state = state.clone();

        for symbol in self.axiom.iter() {
            match symbol.evaluate(&current_state) {
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
                    stack.push(current_state.clone());
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
/// Can run a specific test like this
// cargo test lsystem::tests::produce_and_display -- --exact --show-output

#[cfg(test)]
mod tests {
    use super::*;

    //////////////// Implementation  ////////////////
    use crate::state::rx;
    use crate::state::tx;

    #[derive(Debug, Copy, Clone)]
    struct Constants {
        pub r: f64,
        pub p: f64,
    }

    impl ConstantsTrait for Constants {}

    // #[state::op(RotateX(A))] // Or something
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

    impl AlphabetTrait<Constants> for Alphabet {
        fn produce(&self, constants: &Constants) -> Vec<Alphabet> {
            let r = constants.r;
            let p = constants.p;

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

        fn evaluate(&self, state: &State) -> EvalReturns {
            match self {
                A { s } => EvalReturns::State(rx(state, s.clone())),
                F { x } => EvalReturns::State(tx(state, x.clone())),
                DropF { x } => EvalReturns::DropState(tx(state, x.clone())),
                Push => EvalReturns::PushState,
                Pop => EvalReturns::PopState,
            }
        }
    }

    #[test]
    fn drop_state() {
        let constants = Constants { r: 1.456, p: 1.414 };

        let s0 = 1.0;

        let axiom = vec![A { s: s0 }];

        let lsystem = LSystem {
            constants: constants,
            axiom: axiom,
        };

        println!("{lsystem}");
    }

    #[test]
    fn clone_and_produce() {
        let constants = Constants { r: 1.456, p: 1.414 };

        let s0 = 1.0;

        let axiom = vec![A { s: s0 }];

        let lsystem = LSystem {
            constants: constants,
            axiom: axiom,
        };

        let lsystem_clone = lsystem.clone();

        // Production 0 (Axiom)
        assert_eq!(lsystem_clone.axiom, lsystem.axiom);

        // Production 1
        assert_eq!(lsystem_clone.produce(1).axiom, lsystem.produce(1).axiom);
    }

    #[test]
    fn produce_and_display() {
        // If you want to see the println! outputs, run with
        // cargo test -- --show-output
        // or
        // cargo test -- --nocapture

        let constants = Constants { r: 1.456, p: 1.414 };

        let r = constants.r;
        let p = constants.p;

        let s0 = 1.0;

        let axiom = vec![A { s: s0 }];

        let mut lsystem = LSystem {
            constants: constants,
            axiom: axiom,
        };

        println!("{lsystem}");

        assert_eq!(format!("{lsystem}"), format!("{}", A { s: s0 }));

        lsystem = lsystem.produce(1);

        println!("{lsystem}");

        assert_eq!(
            format!("{lsystem}"),
            format!(
                "{}{}{}{}{}{}",
                F { x: s0 },
                Push,
                A { s: s0 / r },
                Pop,
                DropF { x: s0 },
                A { s: p }
            )
        );

        lsystem = lsystem.produce(1);

        println!("{lsystem}");

        assert_eq!(
            format!("{lsystem}"),
            format!(
                "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                F { x: s0 },
                Push,
                F { x: s0 / r },
                Push,
                A { s: s0 / r / r },
                Pop,
                DropF { x: s0 / r },
                A { s: p },
                Pop,
                DropF { x: s0 },
                F { x: p },
                Push,
                A { s: p / r },
                Pop,
                DropF { x: p },
                A { s: p }
            )
        );
    }

    #[test]
    fn produce() {
        let constants = Constants { r: 1.456, p: 1.414 };

        let r = constants.r;
        let p = constants.p;

        let s0 = 1.0;

        let axiom = vec![A { s: s0 }];

        let mut lsystem = LSystem {
            constants: constants,
            axiom: axiom,
        };

        // Production 0 (axiom)
        assert_eq!(lsystem.axiom, vec![A { s: s0 }]);

        // Production 1
        lsystem = lsystem.produce(1);

        assert_eq!(
            lsystem.axiom,
            vec![
                F { x: s0 },
                Push,
                A { s: s0 / r },
                Pop,
                DropF { x: s0 },
                A { s: p }
            ]
        );

        // Production 2
        lsystem = lsystem.produce(1);

        assert_eq!(
            lsystem.axiom,
            vec![
                F { x: s0 },
                Push,
                F { x: s0 / r },
                Push,
                A { s: s0 / r / r },
                Pop,
                DropF { x: s0 / r },
                A { s: p },
                Pop,
                DropF { x: s0 },
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
    fn produce_and_evaluate() {
        let constants = Constants { r: 1.456, p: 1.414 };

        let r = constants.r;
        let p = constants.p;

        let s0 = 1.0;

        let axiom = vec![A { s: s0 }];

        let mut lsystem = LSystem {
            constants: constants,
            axiom: axiom,
        };

        let state0 = State::from(String::from("S1"));

        // Production 0 (Axiom)
        let states0 = lsystem.evaluate(&state0);

        assert_eq!(
            states0,
            vec![state0.clone(), A { s: s0 }.evaluate(&state0).unwrap_state()]
        );

        // Production 1
        lsystem = lsystem.produce(1);

        let states1 = lsystem.evaluate(&state0);

        // F{x:s0}, Push, A{s:s0/r}, Pop, DropF{x: s0}, A{s:p}

        let state1 = F { x: s0 }.evaluate(&state0).unwrap_state(); // F{x:s0}
        let state2 = A { s: s0 / r }.evaluate(&state1).unwrap_state(); // A{s:s0/r}}
        let state3 = DropF { x: s0 }.evaluate(&state1).unwrap_state(); // f{x:s0}
        let state4 = A { s: p }.evaluate(&state3).unwrap_state(); // A{s:p}

        assert_eq!(states1, vec![state0, state1, state2, state4]); // Notice that state3 has been dropped
    }
}
