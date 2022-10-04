use std::str;
use std::io::{self, BufWriter, BufRead, prelude::*};
use std::collections::HashSet;
 
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
 
  let (n, mut m) = (s.cin::<usize>(), s.cin::<usize>());
 
  let mut values: Vec<usize> = vec![0; n+1];
  let mut pos: Vec<usize> = vec![0; n+1];
 
  for i in 1..=n{
    values[i] = s.cin();
    pos[values[i]] = i;
  }
 
  let mut ans: usize = 1;
 
  for i in 1..n{
    if pos[i] > pos[i+1]{
      ans += 1;
    }
  }
  while m > 0{
 
    let left: usize = s.cin();
    let right: usize = s.cin();
    let mut update_pair: HashSet<(usize, usize)> = HashSet::new();
 
    if values[left]+1 <= n{
      update_pair.insert((values[left], values[left]+1));
    }
    if values[left] -1 >= 1{
      update_pair.insert((values[left]-1, values[left]));
    }
    if values[right]+1 <= n{
      update_pair.insert((values[right], values[right]+1));
    }
    if values[right]+1 >= 1{
      update_pair.insert((values[right]-1, values[right]));
    }
    for &val in update_pair.iter(){
      if pos[val.0] > pos[val.1]{
        ans -= 1;
      }
    }
    values.swap(left, right);
    pos[values[left]] = left;
    pos[values[right]] = right;
 
    for &val in update_pair.iter(){
      if pos[val.0] > pos[val.1]{
        ans += 1;
      }
    }
 
    writeln!(out, "{}", ans)?;
 
    m-=1;
  }
 
  Ok(())
}