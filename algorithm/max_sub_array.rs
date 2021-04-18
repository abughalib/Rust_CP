#![allow(dead_code)]
#![allow(unused_imports)]

use std::{io, str::FromStr, usize};
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

fn find_max_crossing_subarray(vec: &Vec<i32>, low: usize, mid: usize, high: usize)->(usize, usize, i32){
  let mut left_sum: i32 = i32::MIN;
  let mut sum: i32 = 0;
  let mut max_left: usize = 0;

  let mut i: usize = mid.clone();
  while i >= low{
    sum += vec[i];
    if sum > left_sum{
      left_sum = sum;
      max_left = i;
    }
    if i == 0{break;}
    i-=1;
  }
  
  let mut right_sum: i32 = i32::MIN;
  let mut sum: i32 = 0;
  let mut max_right: usize = 0;

  i = mid.clone()+1;
  while i <= high{
    sum += vec[i];
    if sum > right_sum{
      right_sum = sum;
      max_right = i;
    }
    i+=1;
  }

  (max_left, max_right, left_sum+right_sum)
}

fn find_max_subarray(vec: &Vec<i32>, low: usize, high: usize)->(usize, usize, i32){
  let mid = high-(high-low)/2;
  if high == low{
    return (low, high, vec[low]);
  }else if mid == ((low+high) as f32/2.0).floor() as usize {
    let (left_low, left_high, left_sum) = find_max_subarray(&vec, low, mid);
    let (right_low, right_high, right_sum) = find_max_subarray(&vec, mid+1, high);
    let (cross_low, cross_high, cross_sum) = find_max_crossing_subarray(&vec, low, mid, high);

    if left_sum > right_sum && left_sum > cross_sum{
      return (left_low, left_high, left_sum);
    }
    else if right_sum > left_sum && right_sum > cross_sum{
      return (right_low, right_high, right_sum);
    }else{
      return (cross_low, cross_high, cross_sum);
    }
  }

  (0, 0, 0)
}

fn main(){

  let ans = find_max_subarray(&vec![-11, 2, 3, 4, 5], 0, 4);
  println!("Start: {}, End: {}, Sum: {}", ans.0, ans.1, ans.2);

}
