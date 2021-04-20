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

fn get_solution(mut weights: Vec<usize>, x: usize)->usize{
  
  let mut count: usize = 0;
  
  let mut considered: HashSet<usize> = HashSet::new();
  
  weights.sort();
  
  let (mut i, mut j) = (0usize, weights.len()-1);
  
  while i < j{
    if weights[i] + weights[j] <= x{
      considered.insert(i);
      considered.insert(j);
      count+=1;
      i+=1;
      j-=1;
    }
    else{
      j-=1;
    }
  }
  
  if considered.len() != weights.len(){
    for k in i..weights.len(){
      match considered.get(&k){
        Some(_)=>continue,
        None=>count+=1
      }
    }
  }
  
  count
}

fn get_solution_alt(mut weights: Vec<usize>, x: usize)->usize{
  
  let mut count: usize = 0;
  
  weights.sort();
  
  let (mut l, mut r) = (0i32, (weights.len()-1) as i32);
  
  while l <= r{
    count+=1;
    if weights[l as usize]+weights[r as usize] > x {r-=1}
    else{
      l+=1;
      r-=1;
    }
  }
  
  count
}

fn main(){

  let stdin = io::stdin();

  let mut s = Scanner::new(stdin.lock());
  
  let (n, x) = (s.cin::<usize>(), s.cin::<usize>());
  
  let mut weights: Vec<usize> = Vec::with_capacity(n);
  
  for _ in 0..n{
    weights.push(s.cin::<usize>());
  }
  
  println!("{}", get_solution(weights, x));
  
}


#[test]
fn test_solution(){
  assert_eq!(get_solution(vec![7, 2, 3, 9], 10), 3);
  assert_eq!(get_solution(vec![10, 10, 10, 10], 10), 4);
  assert_eq!(get_solution(vec![1, 2, 3, 4, 4], 4), 4);
  assert_eq!(get_solution(vec![1, 2, 8, 9], 10), 2);
  
  assert_eq!(get_solution_alt(vec![7, 2, 3, 9], 10), 3);
  assert_eq!(get_solution_alt(vec![10, 10, 10, 10], 10), 4);
  assert_eq!(get_solution_alt(vec![1, 2, 3, 4, 4], 4), 4);
  assert_eq!(get_solution_alt(vec![1, 2, 8, 9], 10), 2);
  
}
