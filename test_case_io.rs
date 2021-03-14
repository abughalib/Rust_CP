#![allow(dead_code)]
#![allow(unused_imports)]

use std::io;
use std::str;
use std::cmp::{max, min};

struct Scanner<R>{
	reader: R,
	buffer: Vec<String>
}
impl<R: io::BufRead> Scanner<R>{
	fn new(reader: R)->Scanner<R>{
		Scanner{
			reader: reader,
			buffer: vec![]
		}
	}
	fn cin<T: str::FromStr>(&mut self)->T{
		loop{
			if let Some(token) = self.buffer.pop(){
				return token.parse().ok().expect("Failed to Parse!");
			}
			let mut input: String = String::new();
			self.reader.read_line(&mut input).ok().expect("Failed to Read!");
			self.buffer = input.split_whitespace().rev().map(String::from).collect();
		}
	}
}

fn main(){

	let stdin = io::stdin();
	let mut s = Scanner::new(stdin.lock());

	let mut t: usize = scanner.cin();

	while t > 0{
    
		let (mut n, mut k) = (s.cin::<usize>(), s.cin::<usize>());
    

		t-=1;
	}

}
