#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        x: usize,
    }
    let mut i = 0;
    while i * (i + 1) / 2 < x {
        i += 1;
    }
    println!("{}", i);
}
