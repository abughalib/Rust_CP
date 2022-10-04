use std::io::{self, BufRead, BufWriter, prelude::*};
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
 
fn main()->std::io::Result<()>{
 
  let stdin = io::stdin();
  let stdout = io::stdout();
 
  let mut s = Scanner::new(stdin.lock());
  let mut o = BufWriter::new(stdout);
 
  let n: usize = s.cin::<usize>();
 
  let mut nums: Vec<usize> = vec![0; n+1];
 
  let mut ans: usize = 0;
  
  for _ in 0..n{
    let input: usize = s.cin::<usize>();
    nums[input] = 1;
    if nums[input-1] != 1{
      ans +=1;
    }
  }
 
  writeln!(o, "{}", ans)?;
  
  Ok(())
}