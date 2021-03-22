// For positive integers
fn max_sub_array_product(vec: Vec<i32>)->i32{

  let mut max_ending = 1;
  let mut min_ending = 1;
  let mut max_so_far = 1;
  let mut flag = 1;

  for i in 0..vec.len(){
    if vec[i] > 0{
      max_ending = max_ending*vec[i];
      min_ending = std::cmp::min(min_ending*vec[i], 1);
      flag = 1;
    }else if vec[i] == 0{
      max_ending = 1;
      min_ending = 1;
    }else{
      let mut temp = max_ending;
      max_ending = std::cmp::max(min_ending*vec[i], 1);
      min_ending = temp*vec[i];
    }
    if max_so_far < max_ending{
      max_so_far = max_ending;
    }
  }
  if flag == 0 && max_so_far == 1{
    return 0;
  }
  return max_so_far;
}


fn main(){

  let vec = vec![1, -2, -3, 0, 7, -8, -2];
  
  println!("{}", max_sub_array_product(vec));

}
