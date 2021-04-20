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

fn main(){

	let stdin = io::stdin();
	let mut s = Scanner::new(stdin.lock());

	let mut n: usize = s.cin();
	let sum: usize = n*(n+1)/2;

	if sum^1 == sum+1{
		let mut vec1: Vec<usize> = Vec::new();
		let mut vec2: Vec<usize> = Vec::new();

		println!("YES");
		let mut sum2: usize = 0;
		
		while sum2+n < sum/2{
			sum2+=n;
			if sum2 > sum/2{
				sum2-=n;
				break;
			}
			vec1.push(n);
			n-=1;
		}
		if sum2 != sum/2{
			vec1.push((sum/2)-sum2);
		}
		for i in 1..=n{
			if i == ((sum/2)-sum2){
				continue;
			}
			vec2.push(i);
		}
		println!("{}", vec1.len());
		for i in vec1{print!("{} ", i);}
		println!("\n{}", vec2.len());
		for i in vec2{print!("{} ", i);}
	}else{
		println!("NO");
	}
	
}