// cargo run <- Runs this file 
// Factory pattern? https://stackoverflow.com/questions/65847670/how-to-implement-abstract-factory-in-rust

use lsystems::test_lsystem::test_lsystem; 

//////////////// Macros ////////////////
macro_rules! add_macro {
    ($first_value:expr $(,p,$number:expr)*) => {{
        let mut sum = $first_value; 
        $(
            sum += $number; 
        )*
        sum 
    }}
}


fn main() { 
    println!("Main Function");
    println!("");

    //////////////// Coupled LSystem (non-generic implementation) ////////////////    
    // Example 
    // test_lsystem(); // Don't delete this

    //////////////// Uncoupled LSystem (generic implementation) ////////////////
    println!("{}",add_macro!(1,p,2,p,3));

    //////////////// Extending a Base Alphabet ////////////////
    /// One possible approach to think about 
    use lsystems::alphabet::Alphabet; 

    // #[derive(Alphabet)]
    enum ExtAlphabet {
        Base(Alphabet),     
        X{l: f64}
    }    
    
    use ExtAlphabet::X; 

    impl ExtAlphabet {
        fn print(&self) -> () {
            match self {
                ExtAlphabet::Base(a) => a.print(),
                X{l} => println!("X({:.3})",l),
            }
        }
    }

    let x = X{l:5.0};     
    x.print(); 

    // Maybe a good use of a macro is defining a l-system 
    // Like 
    // lsystem!(A,F,f; F(1)[A(s/r)]fA(p); ... ) ? 
    // Or something, you know. 

}
