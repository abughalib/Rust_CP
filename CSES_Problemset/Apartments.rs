#![allow(dead_code)]
#![allow(unused_imports)]

use std::{io, str::FromStr};
use std::cmp::{max, min};
use std::collections::*;
use std::ops::Bound::Included;

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

fn get_solution(mut desired_size: Vec<i64>, mut available_size: Vec<i64>, k: i64)->usize{
  
  desired_size.sort();
  available_size.sort();

  let mut count: usize = 0;
  let (mut i, mut j) = (0usize, 0usize);

  while i < desired_size.len() && j < available_size.len() {

    if desired_size[i]-k <= available_size[j] && desired_size[i]+k >= 
      available_size[j]{
        i+=1;
        j+=1;
        count+=1;
    }
    else if desired_size[i]-k > available_size[j]{
      j+=1;
    }
    else if desired_size[i]+k < available_size[j]{
      i+=1;
    }
  }

  count
}

fn main(){

  let stdin = io::stdin();

  let mut s = Scanner::new(stdin.lock());
  
  let (n, m, k) = (s.cin::<usize>(), s.cin::<usize>(), s.cin::<i64>());
  
  let mut desired_size: Vec<i64> = Vec::with_capacity(n);
  let mut available_size: Vec<i64> = Vec::with_capacity(m);

  for _ in 0..n{
    desired_size.push(s.cin::<i64>());
  }


  for _ in 0..m{
    available_size.push(s.cin::<i64>());
  }

  println!("{}", get_solution(desired_size, available_size, k));

}


#[test]
fn test_solution(){
  assert_eq!(get_solution(vec![10, 16, 34, 37, 46, 49, 56, 62, 69, 86], vec![7, 9, 43, 47, 50, 62, 71, 71, 83, 95], 0), 1);
  assert_eq!(get_solution(vec![90, 41, 20, 39, 49, 21, 35, 31, 74, 86], vec![14, 24, 24, 7, 82, 85, 82, 4, 60, 95], 10), 6);
  assert_eq!(get_solution(vec![59, 5, 65, 15, 42, 81, 58, 96, 50, 1], vec![18, 59, 71, 65, 97, 83, 80, 68, 92, 67], 1000), 10);
  assert_eq!(get_solution(vec![25, 80, 59, 43, 67, 21, 77, 5, 8, 99], vec![66, 41, 62, 24, 88, 55, 1, 53, 50, 60], 1000000000), 10);
  
}
