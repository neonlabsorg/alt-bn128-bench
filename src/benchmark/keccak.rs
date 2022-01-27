use super::PRECISION;
use crate::significant;
use cpu_time::ProcessTime;
use solana_sdk::keccak;
use tracing::info;

const BENCHMARK_NAME: &str = "keccak256";

/// Runs the keccak benchmark and returns the average elapsed time
pub fn keccak_bench(buffers: &[Vec<u8>]) -> f64 {
    info!("> Start {} benchmark...", BENCHMARK_NAME);

    let now = ProcessTime::try_now().expect("Getting process time failed");
    for b in buffers {
        keccak_run(b);
    }
    let d = now.try_elapsed().expect("Getting process time failed");

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
