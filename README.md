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



