use core::fmt;
use std::format;

use proc_lsystems::lsystem::EvalReturns;
use proc_lsystems::lsystem::State;

fn main() {
    pub trait AlphabetTrait: Clone + fmt::Display {
        // This is a maybe, as in I'm not really sure we need this, but maybe?
        // type ConstantsList: Clone;
        // fn constants(&self) -> Self::ConstantsList;

        fn produce(&self) -> Vec<Self>;

        fn evaluate<'state>(&self, state: &'state State) -> &'state EvalReturns; // Function with lifetime parameter defined here
    }

    pub struct LSystem<S: AlphabetTrait> {
        pub axiom: Vec<S>,
    }

    impl<S: AlphabetTrait> LSystem<S> {
        pub fn evaluate(&self, state: &State) -> Vec<State> {
            let mut states = vec![state.clone()];
            let mut stack: Vec<State> = Vec::new();

            // current_state is a reference to state (&State)
            let mut current_state = state; // = state produces a lifetime problem. Not sure how to resolve.

            for symbol in self.axiom.iter() {
                match symbol.evaluate(current_state) {
                    EvalReturns::State(state) => {
                        states.push(state.clone());
                        current_state = &state; // So this state needs to live as long as current_state, I think?
                    }
                    _ => {}
                }
            }

            states
        }
    }
}
