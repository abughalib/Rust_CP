#![allow(dead_code)]
#![allow(unused_imports)]

use std::{io, ops::Add, str::FromStr};
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

fn solution(mut vec: Vec<usize>)->usize{

  vec.sort();

  let mut count_unique: usize = 1;

  for i in 0..(vec.len()-1){
    if vec[i] == vec[i+1]{
      continue;
    }else{
      count_unique+=1;
    }
  }

  count_unique
}

fn solution_alt()->usize{
  let stdin = io::stdin();
  let mut s = Scanner::new(stdin.lock());

  let n: usize = s.cin();

  let mut uset: HashSet<usize> = HashSet::with_capacity(n);

  for _ in 0..n{
    uset.insert(s.cin::<usize>());
  }
  return uset.len();
}

fn main(){

	// let stdin = io::stdin();

	// let mut s = Scanner::new(stdin.lock());

  // let n: usize = s.cin();

  // let mut vec: Vec<usize> = Vec::with_capacity(n);

  // for _ in 0..n{
  //   vec.push(s.cin::<usize>());
  // }

  println!("{}", solution_alt());

}

#[test]
fn test_cases(){
  assert_eq!(solution(vec![1, 2, 3, 4]), 4);
  assert_eq!(solution(vec![3, 4, 23, 1, 1, 2]), 5);
  assert_eq!(solution(vec![2, 3, 2, 2, 3]), 2);
  assert_eq!(solution(vec![1]), 1);
  assert_eq!(solution(vec![1, 1, 1, 1, 1, 5]), 2);
  assert_eq!(solution(vec![1, 2, 2, 2, 2, 2]), 2);
}
