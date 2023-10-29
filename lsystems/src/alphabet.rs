pub enum Alphabet {
    A{s: f64},
    F{x: f64},
    // B{l: f64},
    // DropF{x: f64},        
    Push, 
    Pop 
}

impl Alphabet {
    pub fn print(&self) -> () {
        match self {
            Alphabet::A{s} => println!("A({:.3})",s),
            Alphabet::F{x} => println!("F({:.3})",x),
            Alphabet::Push => println!("["),
            Alphabet::Pop => println!("]")
        }
    }
}
