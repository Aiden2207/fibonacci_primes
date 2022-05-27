use num_bigint::BigUint;
use std::io::prelude::*;
//note: mostly written by github copilot
fn main() -> Result<()> {
    for (i, n) in fibonacci().zip(1u64..).filter(|(_, n)| is_prime(n)) {
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
        curr: 1.into(),
        next: 1.into(),
    }
}

fn is_prime(n: &BigUint) -> bool {
    if n < &2.into() {
        return false;
    }
    if n == &2.into() || n == &3.into() {
        return true;
    }
    if n % &2.into() == 0.into() {
        return false;
    }
    if n % &3.into() == 0.into() {
        return false;
    }
    let mut i = 5.into();
    while i * i <= n {
        if n % i == 0.into() || n % (i + 2.into()) == 0.into() {
            return false;
        }
        i += 4.into();
    }
    true
}
