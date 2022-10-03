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

fn cal_zero_fact(n: usize, elem: i32)->usize{
	let mut power: usize = 0;

	let mut i: u32 = 1;

	loop{
		let val = n/(elem.pow(i) as usize);
		if val < 1{
			break;
		}
		power +=val;
		i+=1;
	}

	power
}

fn main(){

	let stdin = io::stdin();
	let mut s = Scanner::new(stdin.lock());

	let n: usize = s.cin();

	println!("{}", cal_zero_fact(n, 5));
}

#[test]
fn test_ans(){
	assert_eq!(cal_zero_fact(20, 10), 2);
	assert_eq!(cal_zero_fact(100, 3), 48);
	assert_eq!(cal_zero_fact(130, 5), 32);
}
