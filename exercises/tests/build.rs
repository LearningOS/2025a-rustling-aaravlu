use std::time::{SystemTime, UNIX_EPOCH};

// This build script sets an environment variable TEST_FOO with the current timestamp
fn main() {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // Output the timestamp as an environment variable for the test to use
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // Enable the "pass" feature for tests8 to make the test return early
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
