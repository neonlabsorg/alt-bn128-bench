//! alt-bn128-bench alb-bn128 implementation.
//! Original implementation: solana/programs/bpf_loader/src/syscalls.rs

use solana_rbpf::error::{EbpfError, UserDefinedError};
use solana_rbpf::memory_region::{AccessType, MemoryMapping};
use solana_sdk::alt_bn128::prelude::*;
use solana_sdk::instruction::InstructionError;
use solana_sdk::pubkey::{Pubkey, PubkeyError};
use std::fmt;
use std::mem::{align_of, size_of};
use std::slice::from_raw_parts_mut;
use std::str::{FromStr, Utf8Error};

/// Error definitions
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum SyscallError {
    #[error("{0}: {1:?}")]
    InvalidString(Utf8Error, Vec<u8>),
    #[error("BPF program panicked")]
    Abort,
    #[error("BPF program Panicked in {0} at {1}:{2}")]
    Panic(String, u64, u64),
    #[error("cannot borrow invoke context")]
    InvokeContextBorrowFailed,
    #[error("malformed signer seed: {0}: {1:?}")]
    MalformedSignerSeed(Utf8Error, Vec<u8>),
    #[error("Could not create program address with signer seeds: {0}")]
    BadSeeds(PubkeyError),
    #[error("Program {0} not supported by inner instructions")]
    ProgramNotSupported(Pubkey),
    #[error("{0}")]
    InstructionError(InstructionError),
    #[error("Unaligned pointer")]
    UnalignedPointer,
    #[error("Too many signers")]
    TooManySigners,
    #[error("Instruction passed to inner instruction is too large ({0} > {1})")]
    InstructionTooLarge(usize, usize),
    #[error("Too many accounts passed to inner instruction")]
    TooManyAccounts,
}

/// Errors returned by functions the BPF Loader registers with the VM
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum BpfError {
    VerifierError,
    SyscallError,
}

impl fmt::Display for BpfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BpfError::VerifierError => write!(f, "VerifierError"),
            BpfError::SyscallError => write!(f, "SyscallError"),
        }
    }
}

impl UserDefinedError for BpfError {}

impl From<SyscallError> for BpfError {
    fn from(_error: SyscallError) -> Self {
        BpfError::SyscallError
    }
}

impl From<SyscallError> for EbpfError<BpfError> {
    fn from(error: SyscallError) -> Self {
        EbpfError::UserError(error.into())
    }
}

/// Error handling for SyscallObject::call methods
macro_rules! question_mark {
    ( $value:expr, $result:ident ) => {{
        let value = $value;
        match value {
            Err(err) => {
                *$result = Err(err.into());
                return;
            }
            Ok(value) => value,
        }
    }};
}

fn translate(
    memory_mapping: &MemoryMapping,
    access_type: AccessType,
    vm_addr: u64,
    len: u64,
) -> Result<u64, EbpfError<BpfError>> {
    memory_mapping.map::<BpfError>(access_type, vm_addr, len)
}

fn translate_slice_inner<'a, T>(
    memory_mapping: &MemoryMapping,
    access_type: AccessType,
    vm_addr: u64,
    len: u64,
    _loader_id: &Pubkey,
    enforce_aligned_host_addrs: bool,
) -> Result<&'a mut [T], EbpfError<BpfError>> {
    if !enforce_aligned_host_addrs && (vm_addr as u64 as *mut T).align_offset(align_of::<T>()) != 0
    {
        return Err(SyscallError::UnalignedPointer.into());
    }
    if len == 0 {
        return Ok(&mut []);
    }
    let host_addr = translate(
        memory_mapping,
        access_type,
        vm_addr,
        len.saturating_mul(size_of::<T>() as u64),
    )?;

    if enforce_aligned_host_addrs && (host_addr as *mut T).align_offset(align_of::<T>()) != 0 {
        return Err(SyscallError::UnalignedPointer.into());
    }
    Ok(unsafe { from_raw_parts_mut(host_addr as *mut T, len as usize) })
}

fn translate_slice_mut<'a, T>(
    memory_mapping: &MemoryMapping,
    vm_addr: u64,
    len: u64,
    loader_id: &Pubkey,
    enforce_aligned_host_addrs: bool,
) -> Result<&'a mut [T], EbpfError<BpfError>> {
    translate_slice_inner::<T>(
        memory_mapping,
        AccessType::Store,
        vm_addr,
        len,
        loader_id,
        enforce_aligned_host_addrs,
    )
}

fn translate_slice<'a, T>(
    memory_mapping: &MemoryMapping,
    vm_addr: u64,
    len: u64,
    loader_id: &Pubkey,
    enforce_aligned_host_addrs: bool,
) -> Result<&'a [T], EbpfError<BpfError>> {
    translate_slice_inner::<T>(
        memory_mapping,
        AccessType::Load,
        vm_addr,
        len,
        loader_id,
        enforce_aligned_host_addrs,
    )
    .map(|value| &*value)
}

// ALT-BN128 Addition
pub struct SyscallAltBn128Addition {
    loader_id: Pubkey,
}

impl SyscallAltBn128Addition {
    pub fn new() -> Self {
        Self {
            loader_id: Pubkey::from_str("Cj9ydNGWLePKRztuE3m3zT1uvj2We517k55vq2e65jtP")
                .expect("Invalid solana_sdk::pubkey::Pubkey from string"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn call(
        &self,
        input_addr: u64,
        input_size: u64,
        result_addr: u64,
        _arg4: u64,
        _arg5: u64,
        memory_mapping: &MemoryMapping,
        result: &mut Result<u64, EbpfError<BpfError>>,
    ) {
        let input = question_mark!(
            translate_slice::<u8>(
                memory_mapping,
                input_addr,
                input_size,
                &self.loader_id,
                true
            ),
            result
        );

        let call_result = question_mark!(
            translate_slice_mut::<u8>(
                memory_mapping,
                result_addr,
                ALT_BN128_ADDITION_OUTPUT_LEN as u64,
                &self.loader_id,
                true,
            ),
            result
        );

        let result_point = match alt_bn128_addition(input) {
            Ok(result_point) => result_point,
            Err(e) => {
                *result = Ok(e.into());
                return;
            }
        };

        if result_point.len() != ALT_BN128_ADDITION_OUTPUT_LEN {
            *result = Ok(AltBn128Error::SliceOutOfBounds.into());
            return;
        }

        call_result.copy_from_slice(&result_point);
        *result = Ok(0);
    }
}

// ALT-BN128 Multiplication
pub struct SyscallAltBn128Multiplication {
    loader_id: Pubkey,
}

impl SyscallAltBn128Multiplication {
    pub fn new() -> Self {
        Self {
            loader_id: Pubkey::from_str("Cj9ydNGWLePKRztuE3m3zT1uvj2We517k55vq2e65jtP")
                .expect("Invalid solana_sdk::pubkey::Pubkey from string"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn call(
        &self,
        input_addr: u64,
        input_size: u64,
        result_addr: u64,
        _arg4: u64,
        _arg5: u64,
        memory_mapping: &MemoryMapping,
        result: &mut Result<u64, EbpfError<BpfError>>,
    ) {
        let input = question_mark!(
            translate_slice::<u8>(
                memory_mapping,
                input_addr,
                input_size,
                &self.loader_id,
                true,
            ),
            result
        );
        let call_result = question_mark!(
            translate_slice_mut::<u8>(
                memory_mapping,
                result_addr,
                ALT_BN128_MULTIPLICATION_OUTPUT_LEN as u64,
                &self.loader_id,
                true,
            ),
            result
        );

        let result_point = match alt_bn128_multiplication(input) {
            Ok(result_point) => result_point,
            Err(e) => {
                *result = Ok(e.into());
                return;
            }
        };

        if result_point.len() != ALT_BN128_MULTIPLICATION_OUTPUT_LEN {
            *result = Ok(AltBn128Error::SliceOutOfBounds.into());
            return;
        }

        call_result.copy_from_slice(&result_point);
        *result = Ok(0);
    }
}

// ALT-BN128 Pairing
pub struct SyscallAltBn128Pairing {
    loader_id: Pubkey,
}

impl SyscallAltBn128Pairing {
    pub fn new() -> Self {
        Self {
            loader_id: Pubkey::from_str("Cj9ydNGWLePKRztuE3m3zT1uvj2We517k55vq2e65jtP")
                .expect("Invalid solana_sdk::pubkey::Pubkey from string"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn call(
        &self,
        input_addr: u64,
        input_size: u64,
        result_addr: u64,
        _arg4: u64,
        _arg5: u64,
        memory_mapping: &MemoryMapping,
        result: &mut Result<u64, EbpfError<BpfError>>,
    ) {
        let input = question_mark!(
            translate_slice::<u8>(
                memory_mapping,
                input_addr,
                input_size,
                &self.loader_id,
                true
            ),
            result
        );
        let call_result = question_mark!(
            translate_slice_mut::<u8>(
                memory_mapping,
                result_addr,
                ALT_BN128_PAIRING_OUTPUT_LEN as u64,
                &self.loader_id,
                true,
            ),
            result
        );

        let result_point = match alt_bn128_pairing(input) {
            Ok(result_point) => result_point,
            Err(e) => {
                *result = Ok(e.into());
                return;
            }
        };

        if result_point.len() != ALT_BN128_PAIRING_OUTPUT_LEN {
            *result = Ok(AltBn128Error::SliceOutOfBounds.into());
            return;
        }

        call_result.copy_from_slice(&result_point);
        *result = Ok(0);
    }
}
