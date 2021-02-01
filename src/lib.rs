//! Foundational crate for retrieving details about the target AVR being compiled for.
//!
//! This crate currently exposes the CPU frequency that is being configured for.
//!
//! # The `AVR_CPU_FREQUENCY_HZ` environment variable
//!
//! All crates that depend on this crate (with the `cpu-frequency` feature enabled) require `$AVR_CPU_FREQUENCY_HZ` when targeting
//! AVR. The frequency will then be available in the `CPU_FREQUENCY_HZ` constant.
//!
//! It is not necessary to set this variable when AVR is not being targeted, for example,
//! when running integration tests on the host machine.

#![no_std]

#[cfg(feature = "cpu-frequency")]
mod cpu_frequency;

#[cfg(feature = "cpu-frequency")]
pub use self::cpu_frequency::*;
