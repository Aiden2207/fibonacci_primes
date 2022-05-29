use nalgebra as na;
use num_bigint::BigUint;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args();
    let path = args.nth(1).unwrap_or_else(|| ".".into());
    let dir = std::fs::canonicalize(&path)
        .context(format!("Failed to canonicalize the path `{path}`"))?;
    let json = dir.join("config.json");
    let config =
        std::fs::File::open(&json).context(format!("Failed to read from `{}`", json.display()))?;
    let config: Config = serde_json::from_reader(config)?;
    let mut results = BTreeMap::new();
    for config in config.competitors {
        let competitor = Competitor::from_config(&config, dir.clone())?;
        let result = competitor.execute().await?;
        results.insert(result, config.name);
    }
    println!("Final results:");
    for ((num, time), name) in results {
        println!("{name}:\n\tnum: {num}\n\ttime: {time:?}");
    }
    Ok(())
}
const INDICES: [u64; 51] = [
    3, 4, 5, 7, 11, 13, 17, 23, 29, 43, 47, 83, 131, 137, 359, 431, 433, 449, 509, 569, 571, 2971,
    4723, 5387, 9311, 9677, 14431, 25561, 30757, 35999, 37511, 50833, 81839, 104911, 130021,
    148091, 201107, 397379, 433781, 590041, 593689, 604711, 931517, 1049897, 1285607, 1636007,
    1803059, 1968721, 2904353, 3244369, 3340367,
];
static FIBS: Lazy<BTreeMap<u64, BigUint>> = Lazy::new(|| {
    INDICES
        .into_iter()
        .map(|n| (n, fibonacci(n as u32)))
        .collect()
});
fn fibonacci(n: u32) -> BigUint {
    let mut mat = na::matrix![BigUint::from(1u32), 1u32.into(); 1u32.into(), 0u32.into()];
    mat.pow_mut(n - 1);
    mat[0].clone()
}

use anyhow::{Context as _, Result};
use std::future::Future;
use std::future::{pending, Pending};
use std::io::{self, ErrorKind};
use std::path::PathBuf;
use std::pin::Pin;
use std::process::Stdio;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio::io::{AsyncRead, AsyncReadExt, ReadBuf};
use tokio::time::{timeout, Timeout};
#[derive(Debug)]
struct ReadTimeout<R> {
    reader: R,
    timeout: Timeout<Pending<()>>,
}
impl<R> ReadTimeout<R> {
    fn new(reader: R, duration: Duration) -> Self {
        Self {
            reader,
            timeout: timeout(duration, pending()),
        }
    }
}

impl<R: AsyncRead> AsyncRead for ReadTimeout<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let inner = unsafe { self.get_unchecked_mut() };
        let res = unsafe { Pin::new_unchecked(&mut inner.timeout) }.poll(cx);
        if res.is_ready() {
            return Poll::Ready(Err(ErrorKind::TimedOut.into()));
        }
        let reader = unsafe { Pin::new_unchecked(&mut inner.reader) };
        reader.poll_read(cx, buf)
    }
}

async fn binary_read_timeout<R: AsyncRead>(
    r: R,
    limit: Duration,
) -> Result<BTreeMap<u64, (Instant, BigUint)>> {
    let r = ReadTimeout::new(r, limit);
    tokio::pin!(r);
    let mut map = BTreeMap::new();
    loop {
        let index = match r.read_u64_le().await {
            Ok(i) => i,
            Err(e) if matches!(e.kind(), ErrorKind::TimedOut | ErrorKind::UnexpectedEof) => {
                return Ok(map);
            }
            Err(e) => Err(e).context("Failed to read from child stdout")?,
        };
        let len = match r.read_u64_le().await {
            Ok(i) => i,
            Err(e) if matches!(e.kind(), ErrorKind::TimedOut | ErrorKind::UnexpectedEof) => {
                return Ok(map);
            }
            Err(e) => Err(e).context("Failed to read from child stdout")?,
        };
        let mut buf = Vec::new();
        match buf.try_reserve(len as _) {
            Ok(_) => (),
            Err(_) => {
                return Ok(map);
            }
        }
        buf.extend(std::iter::repeat(0u8).take(len as _));
        let res = r.read_exact(&mut buf).await;
        match res {
            Ok(_) => {
                let num = BigUint::from_bytes_le(&buf);
                map.insert(index, (Instant::now(), num));
            }
            Err(e) if matches!(e.kind(), ErrorKind::TimedOut | ErrorKind::UnexpectedEof) => {
                return Ok(map);
            }

            Err(e) => Err(e).context("Failed to read from child stdout")?,
        }
    }
}
fn verify_fibs(map: &BTreeMap<u64, (Instant, BigUint)>) -> usize {
    let mut previous = Instant::now() - Duration::from_secs(10_000);
    FIBS.iter()
        .zip(map.iter())
        .take_while(|((i, fib), (j, (time, num)))| {
            if time < &previous {
                return false;
            }
            previous = *time;
            i == j && *fib == num
        })
        .count()
}
use tokio::process::Command;
async fn run_command(mut cmd: Command) -> Result<(usize, Duration)> {
    let start = Instant::now();
    let mut child = cmd.spawn()?;
    let map = binary_read_timeout(
        child.stdout.as_mut().context("child stdout not created")?,
        Duration::from_secs(60),
    )
    .await?;
    child.kill().await?;
    let count = verify_fibs(&map);
    println!("Total count:{}", count);
    for (i, (_, (time, _))) in map.iter().take(count).enumerate() {
        println!("Fibonacci prime {} reached in: {:?}", i, *time - start);
        if i + 1 == count {
            return Ok((i, *time - start));
        }
    }
    unreachable!()
}
async fn setup_command(mut cmd: Command) -> Result<()> {
    cmd.spawn()?.wait().await?;
    Ok(())
}

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
struct CommandConfig {
    command: String,
    #[serde(default)]
    args: Vec<String>,
}
#[derive(Serialize, Deserialize)]
struct CompetitorConfig {
    name: String,
    #[serde(default)]
    setup: Vec<CommandConfig>,
    run: CommandConfig,
}
#[derive(Serialize, Deserialize)]
struct Config {
    competitors: Vec<CompetitorConfig>,
}

struct Competitor {
    setup: Vec<Command>,
    run: Command,
}
impl Competitor {
    fn from_config(config: &CompetitorConfig, mut dir: PathBuf) -> Result<Self> {
        dir.push(&config.name);
        let setup = config
            .setup
            .iter()
            .map(|c| {
                let mut command = Command::new(c.command.as_str());
                command.args(c.args.iter());
                command.current_dir(&dir);
                command.stderr(Stdio::piped());
                command.stdout(Stdio::piped());
                command.stdin(Stdio::null());
                command
            })
            .collect::<Vec<_>>();
        let mut run = Command::new(config.run.command.as_str());
        run.args(config.run.args.iter());
        run.current_dir(dir);
        run.stderr(Stdio::piped());
        run.stdout(Stdio::piped());
        run.stdin(Stdio::null());
        Ok(Self { setup, run })
    }
    async fn execute(self) -> Result<(usize, Duration)> {
        for command in self.setup {
            setup_command(command).await?;
        }
        run_command(self.run).await
    }
}
