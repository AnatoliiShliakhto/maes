fn main() {
    use std::env::var;
    dotenvy::from_path("../.env").ok();
    println!(
        "cargo:rustc-env=APP_NAME={}",
        var("APP_NAME").expect("APP_NAME not found in .env")
    );
    println!(
        "cargo:rustc-env=APP_TITLE={}",
        var("APP_TITLE").expect("APP_TITLE not found in .env")
    );
    println!(
        "cargo:rustc-env=AES_SALT={}",
        var("AES_SALT").expect("AES_SALT not found in .env")
    );
    println!(
        "cargo:rustc-env=AES_KEY={}",
        var("AES_KEY").expect("AES_KEY not found in .env")
    );
    println!("cargo:rerun-if-changed=../.env");
}
