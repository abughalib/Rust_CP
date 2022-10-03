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
 
    let mut t: usize = s.cin();
 
    while t > 0{
 
        let (mut a, mut b) = (s.cin::<usize>(), s.cin::<usize>());
 
        if a < b{
            let tmp = a;
            a = b;
            b = tmp;
        }
 
        if a > 2*b {
            println!("NO");
        }
        else if (a+b)%3 == 0{
            println!("YES");
        }else{
            println!("NO");
        }
 
        t-=1;
    }
 
}