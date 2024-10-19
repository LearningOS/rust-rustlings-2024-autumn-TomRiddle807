//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
fn main() {
    // For tests7
    if let Ok(s) = std::env::var("TEST_FOO") {
        if let Ok(e) = s.parse::<u64>() {
            println!("cargo:rustc-env=TEST_FOO={}", e);
        }
    }

    // For tests8
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
