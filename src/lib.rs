//! Crates for fetching information about the AVR configuration.

#![no_std]

#[allow(unused_imports)] use const_env__value::value_from_env;

pub const CPU_FREQUENCY_HZ: u32 = CPU_FREQUENCY_HZ_IMPL;

/// The default CPU frequency to assume when AVR is not being targeted.
/// This allows the crate to work for tests, and allows generating without
/// targeting AVR.
#[allow(dead_code)]
const DEFAULT_CPU_FREQUENCY_WHEN_NOT_AVR_HZ: u32 = 16_000_000;

#[cfg(target_arch = "avr")]
// N.B. the comment on the end of the next line is there because it will be seen in the compiler diagnostic.
const CPU_FREQUENCY_HZ_IMPL: u32 = value_from_env!("AVR_CPU_FREQUENCY_HZ": u32); // Must be set whenever AVR is being targeted.
#[cfg(not(target_arch = "avr"))]
const CPU_FREQUENCY_HZ_IMPL: u32 = DEFAULT_CPU_FREQUENCY_WHEN_NOT_AVR_HZ;
