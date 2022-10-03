use std::io::{self, BufRead};
use std::str::FromStr;
 
struct Scanner<R>{
    reader: R,
    buffer: Vec<String>
}
 
impl<R: BufRead> Scanner<R>{
    fn new(reader: R)->Self{
        Self{
            reader,
            buffer: vec![]
        }
    }
    fn cin<T: FromStr>(&mut self)->T{
        loop{
            if let Some(t) = self.buffer.pop(){
                return t.parse().ok().expect("Failed to Parse!");
            }
            let mut input: String = String::new();
            self.reader.read_line(&mut input).ok().expect("Failed to Read!");
            self.buffer = input.trim().split_whitespace().rev().map(String::from).collect();
        }
    }
}
 
fn main(){
    let stdin = io::stdin();
    let mut s = Scanner::new(stdin.lock());
 
    let n: usize = s.cin();
 
    println!("0");
 
    for i in 2..=n{
        let total_places = ((i*i)*(i*i-1))/2;
        let total_attack_possible = 4*(i-1)*(i-2);
        println!("{}", total_places-total_attack_possible);
    }
 
}