use std::io::{self, BufRead, BufWriter, prelude::*};
use std::str;
 
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
  fn cin<T: str::FromStr>(&mut self)-> T{
    loop{
      if let Some(t) = self.buf_iter.next(){
	return t.parse().ok().expect("Failed to Parse!");
      }
      self.buf_str.clear();
      self.reader.read_until(b'\n', &mut self.buf_str).ok()
	.expect("Failed to Read!");
      self.buf_iter = unsafe {
	let slice = str::from_utf8_unchecked(&self.buf_str);
	std::mem::transmute(slice.split_whitespace())
      }
    }
  }
}
 
fn main()-> std::io::Result<()>{
 
  let (stdin, stdout) = (io::stdin(), io::stdout());
  let mut s = Scanner::new(stdin.lock());
  let mut out = BufWriter::new(stdout.lock());
 
  let n: usize = s.cin();
 
  let mut coins: Vec<usize> = (0..n).map(|_| s.cin()).collect();
  coins.sort();
 
  let mut coin: usize = 1;
  
  for i in 0..n{
    if coins[i] <= coin{
      coin += coins[i];
    }else{
      break;
    }
  }
 
  writeln!(out, "{}", coin)?;
  
  Ok(())
}