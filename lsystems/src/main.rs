use lsystems::State; 
use lsystems::Constants; 
use lsystems::Alphabet::{A,F}; 
use lsystems::LSystem;

// Factory pattern? https://stackoverflow.com/questions/65847670/how-to-implement-abstract-factory-in-rust

fn main() { 
    let mut lsystem = LSystem{constants: Constants{r:1.456, p:1.414}, axiom: vec![A{s:1.0}]}; 

    println!("{lsystem}");
    println!(); 

    lsystem = lsystem.produce(1); 

    
    println!("{lsystem}");
    println!();
    

    let states = lsystem.evaluate(State::from(String::from("S0")));

    for (i,state) in states.iter().enumerate() {
        println!("S_{i} = {:?}",state)
    }

}
