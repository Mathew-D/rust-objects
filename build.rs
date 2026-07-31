fn main() {
    // Check if we're building for wasm32
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    
    if target_arch == "wasm32" {
        // Allow undefined symbols - they'll be provided by JavaScript at runtime
        println!("cargo:rustc-link-arg=--allow-undefined");
    }
}
