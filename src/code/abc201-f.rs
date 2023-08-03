// https://atcoder.jp/contests/abc201/tasks/abc201_f

const E: isize = 1<<50;

use std::cmp::min;
fn op(a: isize, b: isize) -> isize {
    return min(a, b);
}

struct SegmentTree<D, Op> {
    e: D,
    op: Op,
    sz: usize,
    d: Vec<D>,
}

impl<D: Copy, Op: Fn(D, D) -> D> SegmentTree<D, Op>  {
    fn new(v: &[D], e: D, op: Op) -> Self {
        let n = v.len();
        let mut sz = 1;
        while sz < n {
            sz <<= 1;
        }
        let mut d = vec![e; sz*2];
        for i in 0..n {
            d[sz+i] = v[i];
        }
        for i in (0..sz).rev() {
            d[i] = op(d[i*2], d[i*2+1]);
        }
        SegmentTree {
            e,
            op,
            sz,
            d,
        }
    }

    fn update(&mut self, mut p: usize, x: D) {
        p += self.sz;
        self.d[p] = x;
        while p >= 2 {
            p >>= 1;
            self.d[p] = (self.op)(self.d[p*2], self.d[p*2+1]);
        }
    }

    fn query(&self, mut a: usize, mut b: usize) -> D {
        let mut sml = self.e;
        let mut smr = self.e;
        a += self.sz;
        b += self.sz;
        while a < b {
            if a & 1 > 0 {
                sml = (self.op)(sml, self.d[a]);
                a += 1;
            }
            if b & 1 > 0 {
                b -= 1;
                smr = (self.op)(self.d[b], smr);
            }
            a >>= 1;
            b >>= 1;
        }
        return (self.op)(sml, smr);
    }
}

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: [usize; N],
    }
    let mut idx = vec![0; N+1];
    for i in 0..N {
        idx[P[i]] = i + 1;
    }
    let mut suma = vec![0; N+1];
    let mut sumb = vec![0; N+1];
    let mut sumc = vec![0; N+1];
    for i in 0..N {
        input! {
            A: isize,
            B: isize,
            C: isize,
        }
        suma[i+1] = suma[i] + A;
        sumb[i+1] = sumb[i] + min(A, B);
        sumc[i+1] = sumc[i] + min(A, C);
    }
    let mut Seg = SegmentTree::new(&vec![0; N+1], E, op);
    let mut DP = vec![E; N+1];
    let mut ans = E;
    for i in 1..=N {
        DP[i] = min(sumb[i-1], Seg.query(0, idx[i]) + suma[i-1]);
        ans = min(ans, DP[i] + sumc[N] - sumc[i]);
        Seg.update(idx[i], DP[i] - suma[i]);
    }
    println!("{}", ans);
}