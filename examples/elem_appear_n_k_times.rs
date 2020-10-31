#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

/*Question URL

https://www.geeksforgeeks.org/given-an-array-of-of-size-n-finds-all-the-elements-that-appear-more-than-nk-times/

*/

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

fn elem_count_n_k(vec: &mut Vec<i64>, k: i64)->Vec<(i64, u32)>{

	let count = (vec.len()/k as usize) as u32;

	let mut hash_map: HashMap<i64, u32> = HashMap::with_capacity(vec.len());
	for i in vec{
		let counter = hash_map.entry(*i).or_insert(0);
		*counter+=1;
	}

	let mut elems: Vec<(i64, u32)> = Vec::new();

	for itr in hash_map.iter(){
		if *itr.1 > count{
			elems.push((*itr.0, *itr.1));
		}
	}

	return elems;
}

fn main(){

	let mut vec: Vec<i64> = vec![2, 3, 3, 2];

	println!("{:?}", elem_count_n_k(&mut vec, 3));


}
