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
      self.buffer = input.trim().split_whitespace()
	.rev().map(String::from).collect();
    }
  }
}
 
fn solve(src: i32, dest: i32, aux: i32, disk: i32){
  if disk <= 0{
    return;
  }
  solve(src, aux, dest, disk-1);
  println!("{} {}", src, dest);
  solve(aux, dest, src, disk-1);
}
 
fn main(){
  let stdin = io::stdin();
  let mut s = Scanner::new(stdin.lock());
 
  let n: i32 = s.cin::<i32>();
 
  println!("{}", (1 << n) -1);
 
  solve(1, 3, 2, n);
 
}