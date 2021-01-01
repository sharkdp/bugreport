fn main() {
    // Re-export variables that are only available at build.rs-time, but not
    // at compile time.
    for var in &[
        "PROFILE",
        "TARGET",
        "CARGO_CFG_TARGET_FAMILY",
        "CARGO_CFG_TARGET_OS",
        "CARGO_CFG_TARGET_ARCH",
        "CARGO_CFG_TARGET_POINTER_WIDTH",
        "CARGO_CFG_TARGET_ENDIAN",
        "CARGO_CFG_TARGET_FEATURE",
        "HOST",
    ] {
        println!(
            "cargo:rustc-env=BUGREPORT_{}={}",
            var,
            std::env::var(var).unwrap_or_else(|_| "unknown".into())
        );
    }
}
