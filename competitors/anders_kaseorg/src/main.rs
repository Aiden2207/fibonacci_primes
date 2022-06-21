use pariter::IteratorExt;
use rug::{integer::IsPrime, Integer};
use std::iter;

fn main() {
    println!("3,2");
    println!("4,3");

    let mut fib0 = Integer::from(1);
    let mut fib1 = Integer::from(2);
    let mut n = Integer::from(3);

    for (n, fib) in iter::from_fn(move || {
        let n1 = Integer::from(n.next_prime_ref());
        while n != n1 {
            fib0 += &fib1;
            fib1 += &fib0;
            n += 2;
        }
        Some((n.clone(), fib1.clone()))
    })
    .parallel_filter(|(_, fib)| fib.is_probably_prime(24 + 16) != IsPrime::No)
    {
        println!("{n},{fib}");
    }
}
