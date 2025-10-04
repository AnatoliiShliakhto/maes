fn main() {
    use std::env::var;
    dotenvy::from_path("../.env").ok();
    
    println!(
        "cargo:rustc-env=AES_SALT={}",
        var("AES_SALT").expect("AES_SALT not found in .env")
    );
    println!(
        "cargo:rustc-env=AES_KEY={}",
        var("AES_KEY").expect("AES_KEY not found in .env")
    );

    let rustflags = format!(
        "{rustflags} -C target-cpu=native -C target-feature=+aes,+sse2,+sse4.1,+ssse3",
        rustflags = var("RUSTFLAGS").unwrap_or_default()
    );
    println!("cargo:rustc-env=RUSTFLAGS={}", rustflags);

    println!("cargo:rerun-if-changed=../.env");
}
