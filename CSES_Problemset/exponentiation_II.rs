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

fn solution(mut a: u64, mut b: u64) -> u64 {
    let modulu: u64 = 1e9 as u64 + 7;
    let mut ans: u64 = 1;

    a = a % modulu;

    while b > 0 {
        if b & 1 != 0 {
            ans = (ans * a) % modulu;
        }

        b = b >> 1;
        a = (a * a) % modulu;
    }

    return ans;
}

fn main() {
    let stdin = io::stdin();
    let mut s = Scanner::new(stdin.lock());
    let mut t: usize = s.cin();

    while t > 0 {
        let (a, b) = (s.cin::<u64>(), s.cin::<u64>());

        println!("{}", solution(a, b));

        t -= 1;
    }
}

#[test]
fn test_cases() {
    assert_eq!(solution(5, 3), 125 % (10e9 as u64 + 7));
    assert_eq!(solution(123, 123), 921450052);
}
