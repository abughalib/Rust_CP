fn lower_bound(vec: &Vec<usize>, target: usize)->usize{
  
  let mut start = 0;
  let mut end = vec.len()-1;
  let mut ans: usize = 0;
  
  while start <= end{
    let mid = end - ((end-start)/2);
    if vec[mid] == target{
      return target;
    }
    else if vec[mid] > target{
      end = mid-1;
    }else{
      ans = vec[mid];
      start = mid+1;
    }
  }
  return ans;
}

  fn main(){
  
  
  }
  
#[test]
fn test_solution(){
  
  assert_eq!(lower_bound(&vec![1, 2, 3, 4, 5, 6, 7], 5), 5);
  assert_eq!(lower_bound(&vec![1, 2, 3, 4, 6, 7], 5), 4);
  assert_eq!(lower_bound(&vec![1, 2, 3, 6, 7], 5), 3);
  
  assert_eq!(lower_bound(&vec![1, 2, 99, 100, 105, 200], 5), 2);
  
}
