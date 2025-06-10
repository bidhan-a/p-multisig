#![no_std]

pub mod constants;
pub mod error;
pub mod instruction;
pub mod state;

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "no-bpf-entrypoint"))]
mod entrypoint;

pinocchio_pubkey::declare_id!("3F4YpPhFJo7BjAApz8Zbigxbbp4RBK1UxTBdYdC8M6Uq");
