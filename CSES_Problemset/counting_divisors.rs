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
            reader,
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

fn solution(x: usize) -> usize {
    let mut count: usize = 0;

    for i in 1..=((x as f64).sqrt() as usize) {
        if x % i == 0 {
            count += 1;
            if x % (x / i) == 0 && i != x / i {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut s = Scanner::new(stdin.lock());

    let t: usize = s.cin();

    for _ in 0..t {
        let n: usize = s.cin();
        println!("{}", solution(n));
    }
}

#[test]
fn test_cases() {
    assert_eq!(solution(16), 5);
    assert_eq!(solution(17), 2);
    assert_eq!(solution(18), 6);
}
