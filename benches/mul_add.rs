#![feature(test)]
use blast::core::math::{expm1, expm1_mul_add};

extern crate test;
use test::Bencher;

#[bench]
fn bench_expm1(b: &mut Bencher) {
    let v: Vec<f64> = (1..=10000)
        .into_iter()
        .map(|i| i as f64 / 100000.)
        .collect();
    b.iter(|| {
        for &i in &v {
            let x = expm1(i);
            drop(x);
        }
    })
}

#[bench]
fn bench_expm1_mul_add(b: &mut Bencher) {
    let v: Vec<f64> = (1..=10000)
        .into_iter()
        .map(|i| i as f64 / 100000.)
        .collect();
    b.iter(|| {
        for &i in &v {
            let x = expm1_mul_add(i);
            drop(x);
        }
    })
}

#[bench]
fn bench_expm1_native(b: &mut Bencher) {
    let v: Vec<f64> = (1..=10000)
        .into_iter()
        .map(|i| i as f64 / 100000.)
        .collect();
    b.iter(|| {
        for &i in &v {
            let x = f64::exp_m1(i);
            drop(x);
        }
    })
}
