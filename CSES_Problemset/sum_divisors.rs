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

fn power(mut x: i64) -> i64 {
    let modulu: i64 = 1e9 as i64 + 7;
    let mut y: i64 = modulu - 2;
    let mut ans = 1;
    x = x % modulu;
    while y > 0 {
        if y & 1 != 0 {
            ans = (ans * x) % modulu;
        }
        y = y >> 1;
        x = (x * x) % modulu;
    }
    return (ans + modulu) % modulu;
}

fn solution(n: i64) -> i64 {
    let modulu: i64 = 1e9 as i64 + 7;
    let mut l: i64 = 1;
    let mut ans: i64 = 0;

    let sum = |x: i64| return ((((x % modulu) * ((x + 1) % modulu)) % modulu) * power(2)) % modulu;

    while l <= n {
        let mut k: i64 = n / l;
        let r: i64 = n / k;
        k %= modulu;
        ans += ((sum(r) - sum(l - 1) % modulu) * k) % modulu;
        ans %= modulu;
        l = r + 1;
    }
    ans %= modulu;

    if ans < 0 {
        return ans + 1e9 as i64 + 7;
    } else {
        return ans;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut s = Scanner::new(stdin.lock());

    let n: i64 = s.cin();

    let ans: i64 = solution(n);

    if ans < 0 {
        println!("{}", ans + 1e9 as i64 + 7);
    }

    println!("{}", ans);
}

#[test]
fn test_cases() {
    assert_eq!(solution(12), 127);
    assert_eq!(solution(5), 21);
    assert_eq!(solution(6), 33);
    assert_eq!(solution(7), 41);
    assert_eq!(solution(1), 1);
    assert_eq!(solution(8), 56);

    assert_eq!(solution(831367), 465422025);
    assert_eq!(solution(994689), 754399607);
    assert_eq!(solution(902342), 670449361);
}
