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

fn gen_lst(n: usize)->Vec<usize>{
	let mut vec:Vec<usize> = Vec::with_capacity(n);

	let mut init_value = 1;

	while init_value + 2 <= n{
		vec.push(init_value);
		init_value+=2;
	}

	vec.push(init_value);

	let mut idx = 0;

	while vec[idx]+1 <= n{
		vec.push(vec[idx]+1);
		idx+=1;
	}

	vec
}

fn main(){

	let stdin = io::stdin();
	let mut s = Scanner::new(stdin.lock());
	let n: usize = s.cin();
	if n == 1{
		println!("1");
	}else if n < 4{
		println!("NO SOLUTION");
	}else{
		if n == 4{
			println!("{} {} {} {}", 2, 4, 1, 3);
		}else{
			let ans = gen_lst(n);
			for i in 0..n{
				print!("{} ", ans[i]);
			}
		}
	}
}

#[test]
fn check_ans(){
	
	assert_eq!(gen_lst(5)[0..5], vec![1,3,5,2,4]);
	assert_eq!(gen_lst(6)[0..6], vec![1,3,5,2,4,6]);
	assert_eq!(gen_lst(7)[0..7], vec![1,3,5,7,2,4,6]);

}