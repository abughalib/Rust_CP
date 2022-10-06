use std::str;
use std::io::{self, BufWriter, BufRead, prelude::*};
use std::collections::{VecDeque, HashSet};
 
struct Scanner<R>{
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitWhitespace<'static>
}
 
impl<R: BufRead> Scanner<R>{
    fn new(reader: R)-> Self{
        Self{
            reader,
            buf_str: vec![],
            buf_iter: "".split_whitespace()
        }
    }
    fn cin<T: str::FromStr> (&mut self) -> T{
        loop{
            if let Some(t) = self.buf_iter.next(){
                return t.parse().ok().expect("Failed to Parse!");
            }
            self.buf_str.clear();
            self.reader.read_until(b'\n', &mut self.buf_str)
            .expect("Failed to Read!");
            self.buf_iter = unsafe{
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
}
 
 
fn main()->std::io::Result<()>{
 
  let stdin = io::stdin();
  let stdout = io::stdout();
 
  let mut s = Scanner::new(stdin.lock());
  let mut out = BufWriter::new(stdout);
 
  let mut set: HashSet<usize> = HashSet::new();
  let n: usize = s.cin();
  let mut list: VecDeque<usize> = VecDeque::with_capacity(n);
  let mut size: usize = 0;
 
  for _ in 0..n{
 
    let input_val: usize = s.cin();
 
    if set.insert(input_val){
      list.push_back(input_val);
      size = max(size, list.len());
    }else{
      let mut remove_val: usize = list.pop_front().unwrap();
      while list.len() > 0 && remove_val != input_val{
        set.remove(&remove_val);
        remove_val = list.pop_front().unwrap();
      }
      list.push_back(input_val);
      size = max(size, list.len());
    }
 
  }
 
  use std::cmp::max;
 
  writeln!(out, "{}", max(size, list.len()))?;
 
  Ok(())
}