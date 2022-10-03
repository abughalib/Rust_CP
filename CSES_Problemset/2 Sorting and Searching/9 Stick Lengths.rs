use std::io::{self, BufRead, prelude::*, BufWriter};
use std::str::FromStr;
 
struct Scanner<R>{
  reader: R,
  buffer: Vec<String>
}
impl<R: BufRead> Scanner<R>{
  fn new(reader: R)->Self{
    Scanner{
      reader: reader,
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
 
fn main() -> std::io::Result<()>{
 
  let stdin = io::stdin();
  let stdout = io::stdout();
  let mut s = Scanner::new(stdin.lock());
  let mut out = BufWriter::new(stdout);
 
  let n: usize = s.cin();
 
  let mut values: Vec<usize> = (0..n).map(|_| s.cin::<usize>()).collect();
 
  values.sort();
 
  let median: usize = match n % 2 == 0{
    true =>{
      (values[n/2-1]+values[n/2])/2
    },
    false =>{
      values[(n+1)/2-1]
    }
  };
 
  let mut ans: usize = 0;
  
  for val in values{
    ans += (val as i64 - median as i64).abs() as usize;
  }
 
  writeln!(out, "{}", ans)?;
 
  Ok(())
}