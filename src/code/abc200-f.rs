// https://atcoder.jp/contests/abc200/tasks/abc200_f

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

fn bin_power(mut x: usize, mut k: usize) -> usize {
    let mut ret = 1;
    while k > 0 {
        if k & 1 > 0 {
            ret = (ret * x) % MOD;
        }
        x = (x * x) % MOD;
        k >>= 1;
    }
    return ret;
}

fn f(a: char, b: char) -> usize {
    if a == '?' || b == '?' {
        return bin_power(2, MOD-2);
    }
    if a != b {
        return 1;
    }
    return 0;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        K: usize,
    }
    let N = S.len();
    if N == 1 && K == 1 {
        println!("0");
        return;
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut res = 0;
    for i in 0..N {
        res += f(S[i], S[(i+1)%N]) * K;
        res %= MOD;
    }
    let cnt = S.iter().filter(|&x| *x == '?').count();
    res *= bin_power(2, MOD-2);
    res %= MOD;
    res *= bin_power(2, cnt*K);
    res %= MOD;
    println!("{}", res);
}