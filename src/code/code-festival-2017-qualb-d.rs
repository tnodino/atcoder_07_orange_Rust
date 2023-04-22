// https://atcoder.jp/contests/code-festival-2017-qualb/tasks/code_festival_2017_qualb_d

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        s: String,
    }
    let s = format!("0{}0", s);
    let s = s.chars().collect::<Vec<char>>();
    let mut t = Vec::new();
    for i in 1..=N {
        if s[i-1] == '1' && s[i] == '0' && s[i+1] == '1' {
            let mut cnt = 0;
            for l in (0..=i-1).rev() {
                if s[l] == '1' {
                    cnt += 1;
                    t.push((l, i+1, cnt));
                }
                else {
                    break;
                }
            }
            let mut cnt = 0;
            for r in i+1..=N+1 {
                if s[r] == '1' {
                    cnt += 1;
                    t.push((i-1, r, cnt));
                }
                else {
                    break;
                }
            }
        }
    }
    t.sort_by(|a, b| a.1.cmp(&b.1));
    let M = t.len();
    let mut idx = 0;
    let mut DP = vec![0; N+1];
    for i in 1..=N {
        DP[i] = DP[i-1];
        while idx < M && t[idx].1 == i {
            DP[i] = max(DP[i], DP[t[idx].0-1] + t[idx].2);
            idx += 1;
        }
    }
    println!("{}", DP[N]);
}