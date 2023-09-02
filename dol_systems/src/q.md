```rust 
struct F {} 

impl Symbol for F {
    fn produce(&self, _constants: Option<HashMap<String,f64>>) -> Vec<Box<dyn Symbol>> {
        vec![Box::new(*self)]
    }
}
```