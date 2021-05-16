#![allow(unused)]

use test::Bencher;

pub fn sqrt_binary_search(x: f32) -> f32 {
    assert!(x >= 0.0);
    let mut low = 0.0;
    let mut up = x;
    let mut last = 0.0;
    loop {
        let mid = (low + up) / 2.0;
        if (mid - last).abs() <= f32::EPSILON {
            return mid;
        }

        if mid * mid > x {
            up = mid;
        } else {
            low = mid;
        }

        last = mid;
    }
}

pub fn sqrt_newton(x: f32) -> f32 {
    assert!(x >= 0.0);
    if x == 0.0 {
        return x;
    }
    if x == 1.0 {
        return 1.0;
    }
    let mut last = 0.0;
    let mut iter_v = x;
    loop {
        // iter_v = iter_v - (iter_v * iter_v - x) / (2.0 * iter_v);
        iter_v = (iter_v + x / iter_v) / 2.0; //由上式化简得来
        if (iter_v - last).abs() <= f32::EPSILON {
            return iter_v;
        }
        last = iter_v;
    }
}

static X: f32 = 999999999.0f32;

#[bench]
fn bench_std_sqrt(b: &mut Bencher) {
    b.iter(|| X.sqrt());
}

#[bench]
fn bench_sqrt_binary_search(b: &mut Bencher) {
    b.iter(|| sqrt_binary_search(X));
}

#[bench]
fn bench_sqrt_newton(b: &mut Bencher) {
    b.iter(|| sqrt_newton(X));
}
