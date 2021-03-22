
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
