
fn merge_two_sorted_array(arr1:&mut Vec<i64>, arr2:&mut Vec<i64>){
  let mut p1: usize = 0;
  let mut p2: usize = 0;

  while p1 < arr1.len() && p2 < arr2.len(){
    if arr1[p1] >= arr2[p2]{
      arr1.insert(p1, arr2[p2]);
      p2+=1;
    }else if arr1[p1] < arr2[p2]{
      if p1 == arr1.len()-1{
        arr1.push(arr2[p2]);
        p2+=1;
      }
      p1+=1;
    }
  }
}

fn main(){

  
  let mut vec1:Vec<i64> = vec![1, 2, 4, 6];
  let mut vec2:Vec<i64> = vec![3, 5, 7, 8];

  merge_two_sorted_array(&mut vec1, &mut vec2);

  for i in vec1.iter(){
    print!("{}->", i);
  }


}