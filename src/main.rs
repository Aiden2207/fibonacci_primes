use nalgebra as na;
use num::BigUint;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
fn main() {
    let nums = println!("{}", fibonacci(25561));
}
const INDICES: [u32; 36] = [
    3, 4, 5, 7, 11, 13, 17, 23, 29, 43, 47, 83, 131, 137, 359, 431, 433, 449, 509, 569, 571, 2971,
    4723, 5387, 9311, 9677, 14431, 25561, 30757, 35999, 37511, 50833, 81839, 104911, 130021,
    148091,
];
static FIBS: Lazy<BTreeMap<u32, BigUint>> =
    Lazy::new(|| INDICES.into_iter().map(|n| (n, fibonacci(n))).collect());
fn fibonacci(n: u32) -> BigUint {
    let mut mat = na::matrix![BigUint::from(1u32), 1u32.into(); 1u32.into(), 0u32.into()];
    mat.pow_mut(n - 1);
    mat[0].clone()
}
