use std::io::{self, BufRead, prelude::*, BufWriter};
use std::str::FromStr;
use core::ops::Bound::{Unbounded, Included};
use std::collections::BTreeMap;
 
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
 
  let (n, m) = (s.cin::<usize>(), s.cin::<usize>());
 
  let mut prices: BTreeMap<usize, usize> = BTreeMap::new();
 
  for _ in 0..n{
    let price: usize = s.cin::<usize>();
    if let Some(t) = prices.get_mut(&price){
      *t+=1;
    }else{
      prices.insert(price, 1);
    }
  }
 
  for _ in 0..m{
    let max_price: usize = s.cin();
    let value = prices.range((Unbounded, Included(max_price))).rev().next();
    match value{
      Some(t)=>{
        let (val, oc) = (*t.0, *t.1);
        writeln!(out, "{}", val)?;
        if oc > 1{
          match prices.get_mut(&val){
            Some(k)=>{
              *k-=1;
            },
            None=>{}
          }
        }else{
          prices.remove(&val);
        }
      },
      None=>{
        writeln!(out, "-1")?;
      }
    }
 
  }
 
 
  Ok(())
 
}