use super::PRECISION;
use crate::alt_bn128::{BpfError, SyscallAltBn128Multiplication};
use crate::significant;
use cpu_time::ProcessTime;
use solana_rbpf::vm::Config;
use tracing::{error, info};

const BENCHMARK_NAME: &str = "alt_bn128 Multiplication";

/// Runs the alt bn128 Multiplication benchmark
pub fn alt_bn128_bench_multiplication(inputs: &[Vec<u8>], k: f64) {
    info!("> Start {} benchmark...", BENCHMARK_NAME);
    let caller = SyscallAltBn128Multiplication::new();
    let config = Config::default();

    let now = ProcessTime::try_now().expect("Getting process time failed");
    for input in inputs {
        alt_bn128_run_multiplication(&caller, &config, input.as_ref());
    }
    let d = now.try_elapsed().expect("Getting process time failed");

    let nanos = d.as_nanos() as f64;
    let total = nanos / 1E9;
    let n = inputs.len() as f64;
    let average = total / n;

    info!("Finish {}", BENCHMARK_NAME);
    info!(
        "{} ({} executions) elapsed {} s.",
        BENCHMARK_NAME,
        n,
        significant::precision(total, PRECISION)
    );
    info!(
        "{} average: {} s. = {} K",
        BENCHMARK_NAME,
        significant::precision(average, PRECISION),
        significant::precision(average / k, PRECISION)
    );
}

/// Executes single alt_bn128 Multiplication call.
#[inline]
fn alt_bn128_run_multiplication(
    syscall: &SyscallAltBn128Multiplication,
    config: &Config,
    input: &[u8],
) {
    use solana_rbpf::error::EbpfError;
    use solana_rbpf::memory_region::{MemoryMapping, MemoryRegion};

    let memory_mapping = MemoryMapping::new::<BpfError>(
        vec![MemoryRegion::new_from_slice(input, 0, 0, true)],
        config,
    )
    .unwrap();

    let mut result: Result<u64, EbpfError<BpfError>> = Ok(0);
    syscall.call(0, 0, 0, 0, 0, &memory_mapping, &mut result);

    if let Err(err) = result {
        error!("{:?}", err);
        panic!("{:?}", err);
    }
    assert_eq!(result.unwrap(), 0);
}
