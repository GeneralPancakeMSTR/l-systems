use lsystems::state::State; 
use lsystems::lsystem::Constants; 
use lsystems::lsystem::Alphabet::{A,F}; 
use lsystems::lsystem::LSystem;

use lsystems::glsystem::GLSystem; 


use lsystems::galphabet::{self}; 

// Factory pattern? https://stackoverflow.com/questions/65847670/how-to-implement-abstract-factory-in-rust

fn main() { 
    //////////////// lsystem ////////////////
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

    println!(); 

    //////////////// generic (g) lsystem ////////////////
    let gconstants = galphabet::Constants{r: 1.456, p: 1.414}; 
    
    let axiom = vec![galphabet::Alphabet::A{s:1.0}]; 

    let mut glsystem = GLSystem{constants: gconstants, axiom: axiom}; 

    println!("{glsystem}");
    println!();

    glsystem = glsystem.produce(1); 

    println!("{glsystem}");
    println!();

    let s0 = State::from(String::from("S0"));

    let gstates = glsystem.evaluate(s0); 

    for (i,gstates) in gstates.iter().enumerate() {
        println!("S_{i} = {:?}",gstates)
    }

    println!(); 


    

}
