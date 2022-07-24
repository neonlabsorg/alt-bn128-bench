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
        let addition_inputs = generate_buffers_add(count);
        alt_bn128_bench_addition(&addition_inputs, k);
    }

    if bench.is_none() || bench.as_ref().unwrap().eq(&Bench::Multiplication) {
        println!();
        let multiplication_inputs = generate_buffers_mul(count);
        alt_bn128_bench_multiplication(&multiplication_inputs, k);
    }

    if bench.is_none() || bench.as_ref().unwrap().eq(&Bench::Pairing) {
        println!();
        let pairing_inputs = generate_buffers_pair(count);
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

fn generate_buffers_add(count: usize) -> Vec<Vec<u8>> {
    let input_pattern = "18b18acfb4c2c30276db5411368e7185b311dd124691610c5d3b74034e093dc9063c909c4720840cb5134cb9f59fa749755796819658d32efc0d288198f3726607c2b7f58a84bd6145f00c9c2bc0bb1a187f20ff2c92963a88019e7c6a014eed06614e20c147e940f2d70da3f74c9a17df361706a4485c742bd6788478fa17d7";
    let input = array_bytes::hex2bytes_unchecked(input_pattern);
    let mut buffers: Vec<Vec<u8>> = Vec::with_capacity(count);
    for _ in 0..count {
        buffers.push(input);
    }
    buffers
}

fn generate_buffers_mul(count: usize) -> Vec<Vec<u8>> {
    let input_pattern = "2bd3e6d0f3b142924f5ca7b49ce5b9d54c4703d7ae5648e61d02268b1a0a9fb721611ce0a6af85915e2f1d70300909ce2e49dfad4a4619c8390cae66cefdb20400000000000000000000000000000000000000000000000011138ce750fa15c2";
    let input = array_bytes::hex2bytes_unchecked(input_pattern);
    let mut buffers: Vec<Vec<u8>> = Vec::with_capacity(count);
    for _ in 0..count {
        buffers.push(input);
    }
    buffers
}

fn generate_buffers_pair(count: usize) -> Vec<Vec<u8>> {
    let input_pattern = "1c76476f4def4bb94541d57ebba1193381ffa7aa76ada664dd31c16024c43f593034dd2920f673e204fee2811c678745fc819b55d3e9d294e45c9b03a76aef41209dd15ebff5d46c4bd888e51a93cf99a7329636c63514396b4a452003a35bf704bf11ca01483bfa8b34b43561848d28905960114c8ac04049af4b6315a416782bb8324af6cfc93537a2ad1a445cfd0ca2a71acd7ac41fadbf933c2a51be344d120a2a4cf30c1bf9845f20c6fe39e07ea2cce61f0c9bb048165fe5e4de877550111e129f1cf1097710d41c4ac70fcdfa5ba2023c6ff1cbeac322de49d1b6df7c2032c61a830e3c17286de9462bf242fca2883585b93870a73853face6a6bf411198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa";
    let input = array_bytes::hex2bytes_unchecked(input_pattern);
    let mut buffers: Vec<Vec<u8>> = Vec::with_capacity(count);
    for _ in 0..count {
        buffers.push(input);
    }
    buffers
}
