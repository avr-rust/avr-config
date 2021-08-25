fn main() {
    #[cfg(feature = "cpu-frequency")]
    println!("cargo:rerun-if-env-changed=AVR_CPU_FREQUENCY_HZ");
}
