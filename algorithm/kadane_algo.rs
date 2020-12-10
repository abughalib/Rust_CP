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

  let mut t = cin();

  while t > 0{

    let arri = getline();
    let mut global_sum = -10000000;
    let mut local_sum = -10000000;

    for i in arri.iter(){
      local_sum = std::cmp::max(*i, local_sum+*i);
      global_sum = std::cmp::max(global_sum, local_sum);
    }

    println!("{}", global_sum);

    t -= 1;
  }

}
