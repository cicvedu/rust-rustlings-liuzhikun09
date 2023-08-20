use std::time::SystemTime;

fn main() {
    
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("cargo:rustc-env=TEST_FOO={}", timestamp.to_string());
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
