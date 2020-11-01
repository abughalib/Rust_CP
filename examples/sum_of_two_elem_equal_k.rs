#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]


use std::io;
use std::colletions::HashMap;

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
fn sum_of_elem(k:i64, arri:&Vec<i64>)->Vec<i64>{

	let mut vec: Vec<i64> = Vec::new();
	let mut umap: HashMap<i64, bool> = HashMap::new();

	for i in vec.iter(){
		if(umap[k-*i]){
			vec.push(*i);
			vec.push(k-*i);
		}
		else{
			umap[k-*i] = true;
		}
	}
	return vec;
}

fn main(){

	let vec: Vec<i64> = vec![1,2,3,4,5,6,7,8,9,10];

	let ans = sum_of_elem(10, vec);

	if(ans.size() < 2)
		println!("-1");
	else{
		for i in 0..(ans.len()-1){
			println!("{}+{} = {}", ans[i], ans[i+1], 10);
		}
	}

}
