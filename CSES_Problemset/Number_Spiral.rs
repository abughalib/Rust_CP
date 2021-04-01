#![allow(dead_code)]
#![allow(unused_imports)]
 
use std::io::{self, Read};
use std::str;
use std::cmp::{max, min};
use std::collections::*;
 
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
 
fn find_value(a: usize, b: usize)->usize{
	let sum_2 = 1+max(a, b)*(max(a,b)+1);
 
	let ans: usize;
 
	if a > b{
		if a^1 == a+1{
			ans = sum_2+(a-b);
		}else{
			ans = sum_2-(a-b);
		}
	}else{
		if b^1 == b+1{
			ans = sum_2-(b-a);
		}else{
			ans = sum_2+(b-a);
		}
	}
	ans
}
 
fn main(){
 
	let stdin = io::stdin();
	let mut s = Scanner::new(stdin.lock());
 
	let mut t: usize = s.cin();
 
	while t > 0{
 
		let (a, b) = (s.cin::<usize>(), s.cin::<usize>());
 
		println!("{}", find_value(b-1, a-1));
 
		t-=1;
	}
 
}
 
#[test]
fn check_ans(){
	assert_eq!(find_value(1, 1), 3);
	assert_eq!(find_value(2, 3), 14);
	assert_eq!(find_value(3, 3), 13);
	assert_eq!(find_value(3, 4), 20);
	assert_eq!(find_value(4, 3), 22);
	assert_eq!(find_value(2, 4), 19);
	assert_eq!(find_value(0, 4), 17);
	assert_eq!(find_value(2, 0), 9);
 
	println!("{}", std::usize::MAX);
}
