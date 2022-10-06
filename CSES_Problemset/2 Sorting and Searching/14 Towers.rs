use std::io::{self, BufRead, prelude::*, BufWriter};
use std::str::FromStr;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Unbounded};
 
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
 
  let mut tree: BTreeMap<usize, usize> = BTreeMap::new();
  let mut box_len: usize = 0;
 
  for _ in 0..n{
    let num: usize = s.cin();
    let value = tree.range((Excluded(num), Unbounded)).next();
    if value.is_some(){
      let (val, oc) = (*value.unwrap().0, *value.unwrap().1);
      tree.remove(&val);
      if oc > 1{
        tree.insert(val, oc-1);
      }
      tree.insert(num, 1);
    }else{
      match tree.get_mut(&num){
        Some(t)=>{
          *t+=1;
        },
        None=>{
          tree.insert(num, 1);
        }
      }
      box_len +=1;
    }
  }
 
  writeln!(out, "{}", box_len)?;
 
  Ok(())
}