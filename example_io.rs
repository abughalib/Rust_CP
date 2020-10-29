#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]


use std::io;

fn cin()->i64{
  let mut inputValue = String::new();
  io::stdin().read_line(&mut inputValue).expect("Enter something");
  return inputValue.trim().parse().unwrap();
}
fn getline()->Vec<i64>{

  let mut inputValue = String::new();
  io::stdin().read_line(&mut inputValue).ok().expect("Reading Error");

  inputValue.split_whitespace()
    .map(|s| s.parse().expect("Parsing Error")).collect()
}

fn main(){

  let mInput = getline();
  let mut k = mInput[0];
  let mut n = mInput[1];

  let mut arr = getline();
  arr.sort();

  for i in 0..(arr.len()/2){
    arr[i] = arr[i] + k;
  }
  for i in arr.len()/2..arr.len(){
    arr[i] = arr[i]-k;
  }
  println!("{}", (arr[arr.len()-1]-arr[0]));

}