#![allow(dead_code)]
#![allow(unused_imports)]
 
use std::{io, str::FromStr};
use std::cmp::{max, min};
use std::collections::*;
 
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
 
fn get_solution(mut cust_time: Vec<(usize, i32)>)->usize{
 
  let mut count: usize = 0;
 
  cust_time.sort();
 
  let mut current: i32 = 0;
 
  for i in cust_time{
    current += i.1;
    count = max(count, current as usize);
  }
 
  
  count
}
 
fn main(){
 
  let stdin = io::stdin();
 
  let mut s = Scanner::new(stdin.lock());
  
  let n: usize = s.cin();
 
  let mut cust_time: Vec<(usize, i32)> = Vec::with_capacity(2*n);
 
  for _ in 0..n{
    cust_time.push((s.cin::<usize>(), 1));
    cust_time.push((s.cin::<usize>(), -1));
  }
 
  println!("{}", get_solution(cust_time));
  
}
 
 
#[test]
fn test_solution(){
 
  assert_eq!(get_solution(vec![(5, 1), (8, -1), (2, 1), (4, -1), (3, 1), (9, -1)]), 2);
  assert_eq!(get_solution(vec![(1, 1), (9, -1), (2, 1), (4, -1), (3, 1), (7, -1), (9, 1), (11, -1)]), 3);
  assert_eq!(get_solution(vec![(3, 1), (4, -1), (5, 1), (9, -1), (1, 1), (2, -1), (2, 1), (5, -1)]), 2);
 
}