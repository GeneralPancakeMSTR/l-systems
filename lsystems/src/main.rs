use lsystems::Constants; 
use lsystems::Alphabet::{A,F}; 
use lsystems::produce; 

// skeletizzle recommends something like 
// enum Symbol {
//     A,
//     F,
//   }
//   impl Symbol {
//     fn operate_on_state(&self, state: State) {
//       match self {
//         Symbol::A => ...
//         ,,,
//       }
//     }
//   }

// Factory pattern? https://stackoverflow.com/questions/65847670/how-to-implement-abstract-factory-in-rust

fn main() { 
    let constants = Constants{r:1.456, p:1.414}; 
    let mut axiom = vec![A{s:1.0}]; 
    
    println!("{:?}",axiom);
    println!(); 

    axiom = produce(&axiom, &constants, 2); 

    println!("{:?}",axiom);

}
