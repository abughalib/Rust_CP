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
 
fn next_permutation<T: std::cmp::Ord>(array: &mut [T]) -> bool {
  // Find non-increasing suffix
  if array.len() == 0 {
      return false;
  }
  let mut i: usize = array.len() - 1;
  while i > 0 && array[i - 1] >= array[i] {
      i -= 1;
  }
  if i == 0 {
      return false;
  }
  
  // Find successor to pivot
  let mut j: usize = array.len() - 1;
  while array[j] <= array[i - 1] {
      j -= 1;
  }
  array.swap(i - 1, j);
  
  // Reverse suffix
  array[i .. ].reverse();
  true
}
 
fn vec_to_string(vec: &Vec<char>)->String{
  let mut output: String = String::with_capacity(vec.len());
  for x in vec.iter(){
    output.push(*x);
  }
  return output;
}
 
fn main() -> std::io::Result<()>{
 
  let stdin = io::stdin();
  let stdout = io::stdout();
  let mut s = Scanner::new(stdin.lock());
  let mut out = BufWriter::new(stdout);
 
  let mut input: Vec<char> = s.cin::<String>().trim().chars().collect();
  input.sort_by(|a , b| (*a as u8).cmp(&(*b as u8)));
 
  let mut ans: Vec<String> = Vec::new();
  ans.push(vec_to_string(&input));
 
  while next_permutation(&mut input){
    ans.push(vec_to_string(&input));
  }
 
  writeln!(out, "{}", ans.len())?;
 
  for pm in ans{
    writeln!(out, "{}", pm)?;
  }
 
  Ok(())
}