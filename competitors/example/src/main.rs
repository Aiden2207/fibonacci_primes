use num_bigint::BigUint;
fn main() {
    for (n, i) in fibonacci().zip(1u64..).filter(|(n, _)| is_prime(n)) {
        println!("{},{}", i, n);
    }
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
