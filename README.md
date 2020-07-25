# avr-config

A foundational crate for retrieving details, such as the CPU clock frequency, at runtime.

[API Documentation](https://docs.rs/avr-config)

## Features

### Retrieving the CPU clock frequency

The `CPU_FREQUENCY_HZ` constant can be used to fetch the target
clock frequency of the AVR. This corresponds to the `$AVR_CPU_FREQUENCY_HZ`
environment variable that must be set when compiling this crate and crates
that use it for AVR.

```rust
fn main() {
    let prescaler = avr_config::CPU_FREQUENCY_HZ / 16;
}
```

No part of the compiler toolchain itself understands or knows about the CPU
frequency of the target device - this is a user-level concern that only affects
user code (such as a busy wait delay loop for timed sleeping).

This crate is not strictly required for retrieving the clock frequency - any logic
or environment variable will do. However, setting up the required clock frequency environment
variables for all your dependent crates could be problematic if there are multiple ways of doing it,
increasing the chance of forgetting one or getting into the situation where the frequencies are
inconsistent and dependent crates execute at different speeds.

This crate aims to provide the boilerplate for getting the CPU frequency as an integer
at runtime, as well as establishes a convention that `$AVR_CPU_FREQUENCY_HZ` is used to
pass the target frequency to all AVR crates, if they opt-in to it.


