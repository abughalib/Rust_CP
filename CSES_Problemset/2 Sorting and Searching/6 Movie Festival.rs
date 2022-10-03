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
 
fn main() -> std::io::Result<()>{
  let stdin = io::stdin();
  let stdout = io::stdout();
  let (mut s, mut o) = (Scanner::new(stdin.lock()), BufWriter::new(stdout));
 
  let n: usize = s.cin::<usize>();
 
  let mut movies: Vec<(usize, usize)> = Vec::with_capacity(n);
 
  for _ in 0..n{
    movies.push((s.cin::<usize>(), s.cin::<usize>()));
  }
 
  movies.sort_by(|x, y| x.1.cmp(&y.1));
 
  let mut count: usize = 0;
  let mut last_finish_time: usize = 0;
 
  for movie in movies{
    if movie.0 >= last_finish_time{
      count+=1;
      last_finish_time = movie.1;
    }
  }
  
  writeln!(o, "{}", count)?;
 
 
  Ok(())
}