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

fn get_solution(inputs: Vec<i64>)->i64{

  let mut global_max: i64 = i64::MIN;
  let mut local_max: i64 = -10e9 as i64;

  for i in inputs{
    local_max = max(i, local_max+i);
    global_max = max(global_max, local_max);
  }
  
  global_max
}

fn main(){

  let stdin = io::stdin();

  let mut s = Scanner::new(stdin.lock());
  
  let n: usize = s.cin();

  let mut inputs: Vec<i64> = Vec::with_capacity(n);

  for _ in 0..n{inputs.push(s.cin::<i64>())}

  println!("{}", get_solution(inputs));
  
}


#[test]
fn test_solution(){

  assert_eq!(get_solution(vec![-1, 3, -2, 5, 3, -5, 2, 2]), 9);
  assert_eq!(get_solution(vec![1, 2, 3, 4, 5, 6]), 21);
  assert_eq!(get_solution(vec![-1, -2, -4, -5, -77, 1, 0]), 1);

}
