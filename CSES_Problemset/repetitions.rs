#![allow(dead_code)]
#![allow(unused_imports)]
 
use std::io::{self, Read};
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
 
	// let stdin = io::stdin();
	// let mut s = Scanner::new(stdin.lock());
 
	let mut seq = String::new();
	io::stdin().read_line(&mut seq).ok().expect("Failed read!");
	
	let mut seq: Vec<char> = seq.chars().collect();
 
	let mut global_max: usize = 0;
	let mut local_max: usize = 0;
 
	for i in 0..(seq.len()-1){
		if seq[i] != seq[i+1]{
			local_max+=1;
			global_max = max(global_max, local_max);
			local_max = 0;
			continue;
		}
		local_max +=1;
	}
 
 
	println!("{}", global_max);
 
}