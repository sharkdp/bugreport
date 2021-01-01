fn main() {
    println!(
        "cargo:rustc-env=BUGREPORT_TARGET={}",
        std::env::var("TARGET").unwrap_or("Unknown".into())
    );
    println!(
        "cargo:rustc-env=BUGREPORT_PROFILE={}",
        std::env::var("PROFILE").unwrap_or("Unknown".into())
    );
}
