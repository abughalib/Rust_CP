
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
