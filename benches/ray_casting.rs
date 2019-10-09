//! Simple benchmark test.

#![allow(dead_code)]
#![allow(clippy::all)]
#![allow(unknown_lints)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![feature(test)]

extern crate test;

fn multiply_by_add(a: i32, b: i32) -> i32 {
    let mut res = 0;

    for _ in 0..a {
        res += b;
    }

    res
}

fn multiply_by_mult(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_multiply_by_add(b: &mut Bencher) {
        b.iter(|| multiply_by_add(34, 25));
    }

    #[bench]
    fn bench_multiply_by_mult(b: &mut Bencher) {
        b.iter(|| multiply_by_mult(34, 25));
    }
}
