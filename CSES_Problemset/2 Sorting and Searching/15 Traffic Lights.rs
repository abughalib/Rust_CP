use std::io::{self, prelude::*, BufRead, BufWriter};
use std::str::*;
use std::collections::{BTreeSet, BTreeMap};
 
struct Scanner<R> {
	reader: R,
	buf_str: Vec<u8>,
	buf_iter: SplitWhitespace<'static>,
}
 
impl<R: BufRead> Scanner<R> {
	fn new(reader: R) -> Self {
		Self {
			reader,
			buf_str: vec![],
			buf_iter: "".split_whitespace(),
		}
	}
 
	fn cin<T: FromStr>(&mut self) -> T {
		loop {
			if let Some(t) = self.buf_iter.next() {
				return t.parse().ok().expect("Failed to Parse!");
			}
			self.buf_str.clear();
			self
				.reader
				.read_until(b'\n', &mut self.buf_str)
				.ok()
				.expect("Failed to Read!");
			self.buf_iter = unsafe {
				let slice = from_utf8_unchecked(&self.buf_str);
				std::mem::transmute(slice.split_whitespace())
			}
		}
	}
}
 
fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let stdout = io::stdout();
 
	let mut s = Scanner::new(stdin.lock());
	let mut out = BufWriter::new(stdout.lock());
 
	let (n, m) = (s.cin::<usize>(), s.cin::<usize>());
 
	let mut set: BTreeSet<i64> = BTreeSet::new();
 
	set.insert(0);
	set.insert(n as i64);
 
	let mut distances: BTreeMap<i64, usize> = BTreeMap::new();
 
	use std::ops::Bound::{Unbounded, Excluded};
 
	distances.insert(n as i64, 1);
 
	for _ in 0..m {
		let input: i64 = s.cin();
 
		set.insert(input);
 
		let lower_bond = set.range((Unbounded, Excluded(input)));
		let mut upper_bond = set.range((Excluded(input), Unbounded));
 
		let lower_val = lower_bond.last().unwrap();
		let higher_val = upper_bond.next().unwrap();
 
		match distances.get_mut(&(higher_val-lower_val)){
			Some(t)=> {
				if *t == 1 {
					distances.remove(&(higher_val-lower_val));
				}else {
					*t-=1;
				}
			},
			None=> {}
		}
 
		match distances.get_mut(&(input-lower_val)) {
			Some(t)=> *t+=1,
			None=>{
				distances.insert(input-lower_val, 1);
			}
		}
 
		match distances.get_mut(&(higher_val - input)) {
			Some(t)=> *t+=1,
			None => {
				distances.insert(higher_val-input, 1);
 
			}
		}
 
		let iter = distances.iter().next_back();
 
		writeln!(out, "{}", iter.unwrap().0)?;
 
	}
 
	Ok(())
}