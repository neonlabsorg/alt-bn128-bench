use super::PRECISION;
use crate::significant;
use solana_sdk::keccak;
use std::time::Instant;
use tracing::info;

const BENCHMARK_NAME: &str = "keccak256";

/// Runs the keccak benchmark and returns the average elapsed time
pub fn keccak_bench(buffers: &[Vec<u8>]) -> f64 {
    info!("> Start {} benchmark...", BENCHMARK_NAME);

    let now = Instant::now();
    for b in buffers {
        keccak_run(b);
    }
    let d = now.elapsed();

    let nanos = d.as_nanos() as f64;
    let total = nanos / 1E9;
    let n = buffers.len() as f64;
    let average = total / n;

    info!("Finish {}", BENCHMARK_NAME);
    info!(
        "{} ({} executions) elapsed {} s.",
        BENCHMARK_NAME,
        n,
        significant::precision(total, PRECISION)
    );
    info!(
        "{} average: {} s.",
        BENCHMARK_NAME,
        significant::precision(average, PRECISION)
    );

    average
}

/// Executes single keccak256 call
#[inline]
fn keccak_run(msg: &[u8]) {
    let _ = keccak::hash(msg);
}
