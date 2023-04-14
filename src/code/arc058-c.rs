// https://atcoder.jp/contests/arc058/tasks/arc058_c

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
        Y: usize,
        Z: usize,
    }
    let M = X + Y + Z;
    let mut mask = 0;
    mask |= 1 << (X + Y + Z - 1);
    mask |= 1 << (Y + Z - 1);
    mask |= 1 << (Z - 1);
    let mut DP = vec![vec![0; 1<<M]; N+1];
    DP[0][0] = 1;
    for i in 0..N {
        for bit in 0..1<<M {
            for a in 1..=10 {
                let mut x = bit;
                x <<= a;
                x |= 1 << (a - 1);
                x %= 1 << M;
                if x & mask == mask {
                    continue;
                }
                DP[i+1][x] += DP[i][bit];
                DP[i+1][x] %= MOD;
            }
        }
    }
    let mut ans = 1;
    for _ in 0..N {
        ans *= 10;
        ans %= MOD;
    }
    for i in 0..1<<M {
        ans = (ans + MOD - DP[N][i]) % MOD;
    }
    println!("{}", ans);
}