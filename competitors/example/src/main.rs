use num_bigint::BigUint;
use std::io::prelude::*;
use std::io::Result;
fn main() -> Result<()> {
    for (n, i) in fibonacci().zip(1u64..).filter(|(n, _)| is_prime(n)) {
        let bytes = n.to_bytes_le();
        let mut lock = std::io::stdout().lock();
        lock.write_all(&i.to_le_bytes())?;
        lock.write_all(&bytes.len().to_le_bytes())?;
        lock.write_all(&bytes)?;
        lock.flush()?;
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
        let result = &self.next + &self.curr;
        let tmp = std::mem::replace(&mut self.next, result);
        let ret = std::mem::replace(&mut self.curr, tmp);
        Some(ret)
    }
}

fn fibonacci() -> impl Iterator<Item = BigUint> {
    Fibonacci {
        curr: 1u32.into(),
        next: 1u32.into(),
    }
    .take(100)
}

fn is_prime(n: &BigUint) -> bool {
    let bound = n.sqrt();
    let mut i: BigUint = 2u32.into();
    if n < &2u32.into() {
        return false;
    }
    while i <= bound {
        if n % &i == 0u32.into() {
            return false;
        }
        i += 1u32;
    }
    true
}
