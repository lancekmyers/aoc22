use std::fs;
use std::path::Path;
pub trait Soln {
    type I;
    type A : std::fmt::Display; 
    type B : std::fmt::Display; 
    const NAME : &'static str;
    fn soln_a(&self, input : &Self::I) -> Self::A;
    fn soln_b(&self, input : &Self::I) -> Self::B;
    fn parse(&self, file : &mut std::fs::File) -> Self::I;

    fn run(&self, path : &str) {
        let path = Path::new(path);
        let mut file = fs::File::open(path).unwrap();
        let input = self.parse(&mut file);
        
        println!("Solution for {}", Self::NAME);
        
        let a = self.soln_a(&input);
        let b = self.soln_b(&input);
        
        println!("\tPart A: {a}\n\tPart B: {b}")
    }
}
