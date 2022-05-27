use num_bigint::BigUint;
use std::io::prelude::*;
use std::io::Result;
//note: mostly written by github copilot
fn main() -> Result<()> {
    for (n, i) in fibonacci().zip(1u64..).filter(|(n, _)| is_prime(n)) {
        let bytes = n.to_bytes_le();
        let mut lock = std::io::stdout();
        lock.write_all(&i.to_le_bytes())?;
        lock.write_all(&bytes.len().to_le_bytes())?;
        lock.write_all(&bytes)?;
    }
    Ok(())
}

//iterator for fibonacci numbers
pub struct Fibonacci {
    curr: BigUint,
    next: BigUint,
}
impl Iterator for Fibonacci {
    type Item = BigUint;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.curr.clone();
        self.curr = self.next.clone();
        self.next += &self.curr;
        Some(result)
    }
}

fn fibonacci() -> impl Iterator<Item = BigUint> {
    Fibonacci {
        curr: 1u32.into(),
        next: 1u32.into(),
    }
}

fn is_prime(n: &BigUint) -> bool {
    if n < &2u32.into() {
        return false;
    }
    if n == &2u32.into() || n == &3u32.into() {
        return true;
    }
    if n % &2u32 == 0u32.into() {
        return false;
    }
    if n % &3u32 == 0u32.into() {
        return false;
    }
    let mut i = 5u32;
    while n >= &(i * i).into() {
        if n % i == 0u32.into() || n % (i + 2) == 0u32.into() {
            return false;
        }
        i += 4u32;
    }
    true
}
