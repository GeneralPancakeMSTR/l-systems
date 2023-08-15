use std::{collections::HashMap, hash::Hash}; 
// use std::rc::Rc; 
use std::fmt;

fn main() {
    // G = <V, w, P> 
    // DOL-System = <Alphabet, Axiom, Productions> 
    
    
    #[derive(Hash, Eq, PartialEq)]
    struct Predecessor(String);

    impl fmt::Display for Predecessor {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f,"{}",self.0)
        }
    }

    struct Successor<'a>(Vec<&'a Predecessor>); 
    // struct Successor(Vec<Rc<Predecessor>>);
    
    
    let a_r = Predecessor(String::from("a_r")); 
    let a_l = Predecessor(String::from("a_l")); 
    let b_r = Predecessor(String::from("b_r")); 
    let b_l = Predecessor(String::from("b_l")); 

    

    let mut productions: HashMap<&Predecessor,Successor> = HashMap::new(); 
    productions.insert(&a_r,Successor(vec![&a_l,&b_r])); // p1 
    productions.insert(&a_l,Successor(vec![&b_l,&a_r])); // p2 
    productions.insert(&b_r,Successor(vec![&a_r])); // p3 
    productions.insert(&b_l,Successor(vec![&a_l])); // p4

    struct Sentence<'a> (Vec<&'a Predecessor>); 



    let axiom = vec![&a_r]; 
    // println!("{axiom}");
    
    // -> vec![&a_l, &b_r]
    // -> vec![&b_l, &a_r, &a_r]
    // -> vec![&a_l, &a_l, &b_r, &a_l, &b_r]
    

    // for reference_to_predecessor in axiom {

    // }    
    

    let reference_to_predecessor = axiom[0]; 


    let mut new_axiom: Vec<&Predecessor> = Vec::new(); 
    let successor = productions.get(reference_to_predecessor); 
    match successor {
        Some(Successor(v)) => {for reference_to_predecessor in v {
            new_axiom.push(*reference_to_predecessor); 
        }},
        None => new_axiom.push(reference_to_predecessor)
    };

    

    




    // struct Production<'a> {
    //     predecessor: &'a Predecessor,
    //     successor: Successor<'a>
    // }

    // let p1 = Production {
    //     predecessor:&a_r,
    //     successor:Successor(vec!(&a_l,&b_r))
    // };

    // let p2 = Production {
    //     predecessor:&a_l,
    //     successor:Successor(vec!(&b_l,&a_r))
    // };

    // let p3 = Production {
    //     predecessor:&b_r,
    //     successor:Successor(vec!(&a_r))
    // };

    // let p4 = Production {
    //     predecessor:&b_l,
    //     successor:Successor(vec!(&a_l))
    // };



    

    
    
    // let mut productions: HashMap<String, Vec<String>> = HashMap::new(); 
    // productions.insert(String::from("a_r"),vec!(String::from("a_l"),String::from("b_r")));
    // productions.insert(String::from("a_l"),vec!(String::from("b_l"),String::from("a_r")));
    // productions.insert(String::from("b_r"),vec!(String::from("a_r")));
    // productions.insert(String::from("b_l"),vec!(String::from("a_l")));

    // let mut axiom = vec!(String::from("a_r"));

    // let axiom = vec!(Predecessor(vec!(String::from("a_r"))));

    // Predecessor is an &Vec<String>
    // for predecessor in axiom.iter(){
    //     let successor = match productions.get(predecessor) {
    //         Some(s) => s,
    //         None => vec!(predecessor),
    //     }; 
    // };

}

// let mut axiom = "a_r";     

// axiom = "abcdef"; 
// println!("{axiom}");


// fn main() {
//     let a = 0;
//     let b = 5;
//     let mut axiom = &a; 
//     println!("{axiom}");
//     axiom = &b; 
//     println!("{axiom}");
//     println!("{a} is unchanged");
//     println!("{b} is still there");
// }