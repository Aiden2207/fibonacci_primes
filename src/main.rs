use nalgebra as na;
use num::BigUint;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
fn main() {
    println!("{}", fibonacci(25561));
}
const INDICES: [u32; 51] = [
    3, 4, 5, 7, 11, 13, 17, 23, 29, 43, 47, 83, 131, 137, 359, 431, 433, 449, 509, 569, 571, 2971,
    4723, 5387, 9311, 9677, 14431, 25561, 30757, 35999, 37511, 50833, 81839, 104911, 130021,
    148091, 201107, 397379, 433781, 590041, 593689, 604711, 931517, 1049897, 1285607, 1636007,
    1803059, 1968721, 2904353, 3244369, 3340367,
];
static FIBS: Lazy<BTreeMap<u32, BigUint>> =
    Lazy::new(|| INDICES.into_iter().map(|n| (n, fibonacci(n))).collect());
fn fibonacci(n: u32) -> BigUint {
    let mut mat = na::matrix![BigUint::from(1u32), 1u32.into(); 1u32.into(), 0u32.into()];
    mat.pow_mut(n - 1);
    mat[0].clone()
}

use std::io::ErrorKind;
use std::io::Result;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio::io::*;
#[derive(Debug)]
struct ReadTimeout<R> {
    reader: R,
    start: Instant,
    timeout: Duration,
}
impl<R> ReadTimeout<R> {
    fn new(reader: R, timeout: Duration) -> Self {
        Self {
            reader,
            start: Instant::now(),
            timeout,
        }
    }
}

impl<R: AsyncRead> AsyncRead for ReadTimeout<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<()>> {
        if Instant::now() - self.start > self.timeout {
            return Poll::Ready(Err(ErrorKind::TimedOut.into()));
        }
        let inner = unsafe { self.get_unchecked_mut() };
        let reader = unsafe { Pin::new_unchecked(&mut inner.reader) };
        reader.poll_read(cx, buf)
    }
}

async fn binary_read_timeout<R: AsyncRead>(
    r: R,
    timeout: Duration,
) -> Result<BTreeMap<u64, (Instant, BigUint)>> {
    let r = ReadTimeout::new(r, timeout);
    tokio::pin!(r);
    let mut map = BTreeMap::new();
    loop {
        let index = match r.read_u64().await {
            Ok(i) => i,
            Err(e) if e.kind() == ErrorKind::TimedOut => {
                return Ok(map);
            }
            Err(e) => return Err(e),
        };
        let len = match r.read_u64().await {
            Ok(i) => i,
            Err(e) if e.kind() == ErrorKind::TimedOut => {
                return Ok(map);
            }
            Err(e) => return Err(e),
        };
        let mut buf = Vec::with_capacity(len as usize);
        let res = r.read_exact(&mut buf).await;
        match res {
            Ok(_) => {
                let num = BigUint::from_bytes_le(&buf);
                map.insert(index, (Instant::now(), num));
            }
            Err(e) if e.kind() == ErrorKind::TimedOut => {
                return Ok(map);
            }

            Err(e) => return Err(e),
        }
    }
}
