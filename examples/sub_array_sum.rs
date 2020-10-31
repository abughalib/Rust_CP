#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]


use std::io;
use std::collections::HashMap;
use std::collections::HashSet;

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

fn sub_array_sum(vec: &mut Vec<i64>, sum:i64)->i64{

	let mut hash_map: HashMap<i64, i64> = HashMap::new();

	let mut current_sum:i64 = 0;

	for i in 0..vec.len(){
		current_sum += vec[i];
		if current_sum == sum{
			println!("{}->{}", 0, i);
			return 1;
		}
		if hash_map.contains_key(&(current_sum-sum)){
			println!("{}->{}", hash_map[&(current_sum-sum)]+1, i);
			return 1;
		}
		hash_map.insert(current_sum, i as i64);
	}

	return 0;
}

fn main(){

	let mut vec: Vec<i64> = vec![-1, -2, 1, 2, 3, 4, 5, 9];

	println!("{:?}", sub_array_sum(&mut vec, 0));


}
