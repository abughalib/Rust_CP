#![allow(dead_code)]
#![allow(unused_imports)]

use std::cmp::{max, min};
use std::collections::*;
use std::{io, ops::Add, str::FromStr};

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

fn solution(nums: &mut Vec<(i64, usize)>, x: i64) -> Option<(usize, usize)> {
    nums.sort_by(|a, b| a.0.partial_cmp(&(b.0)).unwrap());

    let mut right: usize = nums.len() - 1;
    let mut left: usize = 0;

    while left < right {
        if nums[left].0 + nums[right].0 == x {
            return Some((nums[left].1, nums[right].1));
        } else if nums[left].0 + nums[right].0 < x {
            left += 1;
        } else {
            right -= 1;
        }
    }

    None
}

fn main() {
    let stdin = io::stdin();
    let mut s = Scanner::new(stdin.lock());

    let (n, x) = (s.cin::<usize>(), s.cin::<i64>());

    let mut nums: Vec<(i64, usize)> = Vec::with_capacity(n);

    for i in 0..n {
        nums.push((s.cin::<i64>(), i + 1));
    }

    let ans = solution(&mut nums, x);

    match ans {
        Some(t) => {
            println!("{} {}", t.0, t.1);
        }
        None => println!("IMPOSSIBLE"),
    }
}

#[test]
fn test_cases() {
    let mut map: Vec<(i64, usize)> = Vec::new();
    map.push((2, 1));
    map.push((7, 2));
    map.push((5, 3));
    map.push((1, 4));
    map.push((5, 5));

    assert_eq!(solution(&mut map, 8), Some((4, 2)));
    assert_eq!(solution(&mut map, 9), Some((1, 2)));
    assert_eq!(solution(&mut map, 10), Some((3, 5)));
    assert_eq!(solution(&mut map, 99), None);
    assert_eq!(solution(&mut map, 3), Some((4, 1)));
}
