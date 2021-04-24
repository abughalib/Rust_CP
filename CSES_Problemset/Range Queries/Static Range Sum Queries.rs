#![allow(dead_code)]
#![allow(unused_imports)]
 
use std::cmp::{max, min};
use std::collections::*;
use std::{io, str::FromStr, usize};
 
pub struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}
 
impl<R: io::BufRead> Scanner<R> {
    fn new(reader: R) -> Scanner<R> {
        Scanner {
            reader: reader,
            buffer: vec![],
        }
    }
    fn cin<T: FromStr>(&mut self) -> T {
        loop {
            if let Some(t) = self.buffer.pop() {
                return t.parse().ok().expect("Failed to Parse");
            }
            let mut input: String = String::new();
            self.reader
                .read_line(&mut input)
                .ok()
                .expect("Failed to Read!");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}
 
fn solution(nums: &Vec<usize>, a: usize, b: usize) -> usize {
    if a == 0{
        return nums[b];
    }
    return nums[b]-nums[a-1];
}
 
fn main() {
    let stdin = io::stdin();
    let mut s = Scanner::new(stdin.lock());
 
    let (n, q) = (s.cin(), s.cin());
 
    let mut vec: Vec<usize> = Vec::with_capacity(n);
 
    for i in 0..n{
        if i == 0{
            vec.push(s.cin::<usize>());
        }else{
            vec.push(s.cin::<usize>()+vec.last().unwrap());
        }
    }
 
    for _ in 0..q{
        println!("{}", solution(&vec, s.cin::<usize>()-1, s.cin::<usize>()-1));
    }
 
}
 
#[test]
fn test_solution(){
    let vec: Vec<usize> = vec![3, 5, 9, 14, 15, 16, 21, 24];
    assert_eq!(11, solution(&vec, 2-1, 4-1));
    assert_eq!(2, solution(&vec, 5-1, 6-1));
    assert_eq!(24, solution(&vec, 1-1, 8-1));
    assert_eq!(4, solution(&vec, 3-1, 3-1));
 
    let vec1: Vec<usize> = vec![0, 0, 1, 2, 3, 3, 3, 103];
 
    assert_eq!(solution(&vec1, 0, 0), 0);
    assert_eq!(solution(&vec1, 0, 1), 0);
    assert_eq!(solution(&vec1, 0, 2), 1);
    assert_eq!(solution(&vec1, 0, 3), 2);
    assert_eq!(solution(&vec1, 0, 4), 3);
    assert_eq!(solution(&vec1, 2, 4), 3);
    assert_eq!(solution(&vec1, 7, 7), 100);
    assert_eq!(solution(&vec1, 2, 7), 103);
 
}