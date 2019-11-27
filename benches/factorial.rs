#![feature(test)]

extern crate test;

use contracts::pre;
use test::Bencher;

#[bench]
fn bench_factorial_by_recursion(b: &mut Bencher) {
    b.iter(|| factorial_by_recursion(6));
}

#[bench]
fn bench_factorial_by_formula(b: &mut Bencher) {
    b.iter(|| factorial_by_formula(6));
}

#[pre(n >= 1)]
fn factorial_by_recursion(n: i32) -> i32 {
    return if n > 1 {
        n * factorial_by_recursion(n - 1)
    } else {
        1
    };
}

#[pre(n >= 1)]
fn factorial_by_formula(n: i32) -> i32 {
    let a = (n * (n + 1) * (n + 2)) as f64 / 6.0;
    let b = (n * (n + 1)) as f64 / 2.0;
    let xi = 0.630882266676063396815526621896;

    ((2.0f64.powf(a) * xi) % (2.0f64.powf(b))) as i32
}
