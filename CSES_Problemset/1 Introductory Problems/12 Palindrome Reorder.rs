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

fn print_char_times(chr: &char, times: usize)->String{
  let mut result = String::with_capacity(times);
  for _ in 0..times{
    result.push(*chr);
  }
  result
}

fn get_solution(input: String)->String{

  let mut list: HashMap<char, usize> = HashMap::new();

  for chr in input.trim().chars(){
    match list.get_mut(&chr){
      Some(t)=>{*t+=1;},
      None=>{list.insert(chr, 1);}
    }
  }
	
  let mut count_odds: usize = 0;
  let mut odd_value: char = ' ';

  for (key, value) in &list{
    if *value%2 != 0{
      count_odds+=1;
      odd_value = *key;
    }
  }

  if count_odds <= 1{
    let mut final_ans: String = String::with_capacity(input.len());
    for (key, value) in &list{
      final_ans += print_char_times(key, *value/2).as_str();
    }
    if count_odds == 1{
      final_ans.push(odd_value);
    }
    let mut clone_final_ans = final_ans.clone();
    if count_odds == 1{
      clone_final_ans.pop();
    }
    while clone_final_ans.len() > 0{
      final_ans.push(clone_final_ans.pop().unwrap());
    }
    return final_ans;
  }else{
    String::from("NO SOLUTION")
  }
}

fn main(){

	// let stdin = io::stdin();

	// let mut s = Scanner::new(stdin.lock());

  let mut input: String = String::new();
  io::stdin().read_line(&mut input).ok().expect("Failed to read input");

  println!("{}", get_solution(input));

}
