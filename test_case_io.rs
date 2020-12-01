#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use std::io;

fn cin()->i32{
  let mut input = String::new();
  io::stdin().read_line(&mut input).ok().unwrap();
  input.trim().parse().ok().unwrap()
}

fn getline()->Vec<i32>{
  let mut input = String::new();
  io::stdin().read_line(&mut input).ok().unwrap();
  input.split_whitespace().map(|x| x.trim().parse().ok().unwrap()).collect()
}
fn long_cin()->i64{
  let mut input = String::new();
  io::stdin().read_line(&mut input).ok().unwrap();
  input.trim().parse().ok().unwrap()
}
fn long_getline()->Vec<i64>{
  let mut input = String::new();
  io::stdin().read_line(&mut input).ok().unwrap();
  input.split_whitespace().map(|x| x.parse().ok().unwrap()).collect()
}

fn main(){



}
