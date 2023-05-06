// https://atcoder.jp/contests/arc052/tasks/arc052_d

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
        M: usize,
    }
    let N: usize = 100_000;
    let mut map = HashMap::new();
    for i in 0..N {
        let mut x = i;
        let mut s = 0;
        while x > 0 {
            s += x % 10;
            x /= 10;
        }
        let x = (i - s) % K;
        *map.entry(x).or_insert(0) += 1;
    }
    let X = M / N;
    let Y = M % N;
    let mut ans: usize = 0;
    for i in 0..X {
        let mut x = i;
        let mut s = 0;
        while x > 0 {
            s += x % 10;
            x /= 10;
        }
        let x = (i * N - s) % K;
        if map.contains_key(&((K - x) % K)) {
            ans += map[&((K-x)%K)];
        }
    }
    let mut x = X;
    let mut s = 0;
    while x > 0 {
        s += x % 10;
        x /= 10;
    }
    let x = (X * N - s) % K;
    for i in 0..=Y {
        let mut y = i;
        let mut s = 0;
        while y > 0 {
            s += y % 10;
            y /= 10;
        }
        let y = (i - s) % K;
        if (x + y) % K == 0 {
            ans += 1;
        }
    }
    println!("{}", ans - 1);
}