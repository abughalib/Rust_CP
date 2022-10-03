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
 
fn generate_grey_code(n: usize){
 
    let mut arr: Vec<String> = Vec::with_capacity(2i32.pow(n as u32) as usize+1);
    arr.push("0".to_string());
    arr.push("1".to_string());
    let mut i: usize = 2;
 
    while i < (1 << n){
 
        let mut j = i-1;
        while j >= 0{
            arr.push(arr[j].clone());
            if j == 0{
                break;
            }
            j-=1;
        }
        for j in 0..i{
            arr[j] = "0".to_string() + &arr[j];
        }
        for j in i..(2*i){
            arr[j] = "1".to_string() + &arr[j];
        }
 
        i = (i << 1);
    }
 
    for i in 0..arr.len(){
        println!("{}", arr[i]);
    }
 
}
 
fn main(){
 
    let stdin = io::stdin();
    let mut s = Scanner::new(stdin.lock());
 
    let n: usize = s.cin();
 
    generate_grey_code(n);
 
}