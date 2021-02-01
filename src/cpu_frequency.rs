#[allow(unused_imports)] use const_env__value::value_from_env;

/// The clock frequency of the current AVR microcontroller (if the `cpu-frequency` crate feature is
/// enabled).
///
/// This value is set at compilation time.
///
/// This value is derived from the `$AVR_CPU_FREQUENCY_HZ` environment variable.
///
/// When this crate is compiled for a non-AVR target, this value simply becomes
/// a reasonable default.
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

#[cfg(test)]
mod test {
    #[test]
    fn cpu_frequency_is_nonzero() {
        assert!(crate::CPU_FREQUENCY_HZ > 0);

    }
}
