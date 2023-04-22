// https://atcoder.jp/contests/arc019/tasks/arc019_4

use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let N = 169;
    let M = 13;
    let mut ans = vec![vec!['.'; N]; N];
    for i in 0..M {
        for j in 0..M {
            for k in 0..M {
                let m = (i * k + j) % M;
                ans[M*i+j][M*k+m] = 'O';
            }
        }
    }
    let N = 150;
    println!("{}", N);
    for i in 0..N {
        println!("{}", ans[i][..N].iter().collect::<String>());
    }
}