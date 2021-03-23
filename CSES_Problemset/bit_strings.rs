#![allow(dead_code)]
#![allow(unused_imports)]

use std::{io, str::FromStr};
use std::cmp::{max, min};

pub struct Scanner<R>{
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
	fn cin<T: FromStr>(&mut self)->T{
		loop{
			if let Some(t) = self.buffer.pop(){
				return t.parse().ok().expect("Failed to Parse")
			}
			let mut input: String = String::new();
			self.reader.read_line(&mut input).ok().expect("Failed to Read!");
			self.buffer = input.split_whitespace().rev().map(String::from).collect();
		}
	}
}

fn power_mod(x:&mut usize, n:&mut usize)->usize{
	let mut ans: usize = 1;
	let modulu: usize = 1000000000+7;

	while *n > 0{

		if *n & 1 != 0{
			ans = (ans**x)%modulu;
		}

		*n = *n >> 1;
		*x = (*x* *x)%modulu;
	}
	ans
}

fn main(){

	let stdin = io::stdin();

	let mut s = Scanner::new(stdin.lock());

	let mut n: usize = s.cin();

	println!("{}", power_mod(&mut 2, &mut n));

}
