use std::io::{self, BufRead, BufWriter, prelude::*};
use std::str::FromStr;
 
struct Scanner<R>{
  reader: R,
  buffer: Vec<String>
}
 
impl<R: BufRead> Scanner<R>{
  fn new(reader: R)->Self{
    Self{
      reader,
      buffer: vec![]
    }
  }
  fn cin<T: FromStr>(&mut self)->T{
    loop{
      if let Some(t) = self.buffer.pop(){
	return t.parse().ok().expect("Failed to Parse!");
      }
      let mut input: String = String::new();
      self.reader.read_line(&mut input).ok()
	.expect("Failed to Read!");
      self.buffer = input.trim().split_whitespace()
	.rev().map(String::from).collect();
    }
  }
}
 
fn _get_number(mut low: usize, mut high: usize, multi: usize, target: usize, subs: usize, no_digits: usize)->(usize, usize){
  while low <= high{
    let mid = low + (high-low)/2;
    let value = (mid - subs)*no_digits + subs;
    if value <= target{
      if value == target || value + multi >= target{
	// println!("{} {} {}", low, high, value);
	return (value, mid);
      }
      low = mid;
    }else{
      high = mid;
    }
    // println!("{} {} {} {}", low, mid, high, value);
  }
  return (0, 0);
}
 
fn main()-> std::io::Result<()>{
 
  let (stdin, stdout) = (io::stdin(), io::stdout());
  let mut s = Scanner::new(stdin.lock());
  let mut out = BufWriter::new(stdout.lock());
 
  let mut t: usize = s.cin();
 
  let mut power_of_ten: Vec<usize> = vec![1; 19];
 
  for i in 1..19{
    power_of_ten[i] = power_of_ten[i-1]*10;
  }
  
  while t > 0{
 
    let index: usize = s.cin();
    let mut digits_so_far: usize = 0;
    let mut digits_before_actual_block: usize = 0;
    let mut num_of_digits: usize = 0;
 
    for i in 1..=18{
      digits_so_far += (power_of_ten[i] - power_of_ten[i-1])*i;
      if digits_so_far >= index {
	num_of_digits = i;
	break;
      }
      digits_before_actual_block += (power_of_ten[i] - power_of_ten[i-1])*i;
    }
    let mut smallest_value = power_of_ten[num_of_digits-1];
    let mut largest_value = power_of_ten[num_of_digits] -1;
    let mut best_value: usize = 0;
    let mut starting_pos_best_value: usize = 0;
 
    while smallest_value <= largest_value{
      let actual_value = smallest_value + (largest_value - smallest_value)/2;
      let starting_pos_actual_value = digits_before_actual_block + 1 +
	(actual_value - power_of_ten[num_of_digits-1])*num_of_digits;
      if starting_pos_actual_value <= index{
	if actual_value > best_value{
	  best_value = actual_value;
	  starting_pos_best_value = starting_pos_actual_value;
	}
	smallest_value = actual_value+1;
      }else{
	largest_value = actual_value -1;
      }
    }
    let ans = format!("{}", best_value).chars().nth(index-starting_pos_best_value).unwrap();
    writeln!(out, "{}", ans)?;
    
    t-=1;
  }
  
  Ok(())
}