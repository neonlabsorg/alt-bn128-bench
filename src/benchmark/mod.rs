//! alt-bn128-bench benchmark module

mod alt_bn128_addition;
mod alt_bn128_multiplication;
mod alt_bn128_pairing;
mod keccak;

use crate::cli::Bench;
use alt_bn128_addition::alt_bn128_bench_addition;
use alt_bn128_multiplication::alt_bn128_bench_multiplication;
use alt_bn128_pairing::alt_bn128_bench_pairing;
use keccak::keccak_bench;
use solana_sdk::alt_bn128::prelude::*;
use tracing::info;

pub const PRECISION: usize = 4;

/// Runs the benchmark
pub fn run(count: usize, size: usize, bench: Option<Bench>) {
    let buffers = generate_buffers(count, size);

    println!();
    let k = keccak_bench(&buffers);

    if bench.is_none() || bench.as_ref().unwrap().eq(&Bench::Addition) {
        println!();
        let addition_inputs = generate_buffers(count, ALT_BN128_ADDITION_OUTPUT_LEN);
        alt_bn128_bench_addition(&addition_inputs, k);
    }

    if bench.is_none() || bench.as_ref().unwrap().eq(&Bench::Multiplication) {
        println!();
        let multiplication_inputs = generate_buffers(count, ALT_BN128_MULTIPLICATION_INPUT_LEN);
        alt_bn128_bench_multiplication(&multiplication_inputs, k);
    }

    if bench.is_none() || bench.as_ref().unwrap().eq(&Bench::Pairing) {
        println!();
        let pairing_inputs = generate_buffers(count, ALT_BN128_PAIRING_ELEMENT_LEN * 12);
        alt_bn128_bench_pairing(&pairing_inputs, k);
    }
}

/// Generates random data
fn generate_buffers(count: usize, message_size: usize) -> Vec<Vec<u8>> {
    info!(
        "Preparing {} buffers of {} bytes each...",
        count, message_size
    );

    let mut buffers: Vec<Vec<u8>> = Vec::with_capacity(count);
    for _ in 0..count {
        buffers.push((0..message_size).map(|_| rand::random::<u8>()).collect());
    }
    buffers
}
