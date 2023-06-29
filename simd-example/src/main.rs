#![feature(portable_simd)]

use std::simd::Simd;

// element-wise addition
fn add(xs: [f32; 16], ys: [f32; 16]) -> Vec<f32>{
    xs.iter().zip(ys).map(|(lhs, rhs)| lhs + rhs).collect()
}

// simd accelerated addition on vectors a multiple of 16
fn simd_add(xs: [f32; 16], ys: [f32; 16]) -> Vec<f32>{   
    let (v0, v1) = (Simd::from(xs), Simd::from(ys));
    (v0 + v1).to_array().to_vec()
}

fn main() {

    let a0: [f32; 16] = [2.0, 0.0, 2.0, 4.0, 2.0, 0.0, 2.0, 4.0, 2.0, 0.0, 2.0, 4.0, 2.0, 0.0, 2.0, 4.0];
    let a1 = [10.0, 9.0, 8.0, 7.0, 10.0, 9.0, 8.0, 7.0, 10.0, 9.0, 8.0, 7.0, 10.0, 9.0, 8.0, 7.0];
    let res1 = add(a0, a1);
    let res2 = simd_add(a0, a1);

    assert_eq!(res1,res2)
}
