fn main() {
    use ::std::env::var;
    dotenvy::from_path("../.env").ok();

    let current_rustflags = var("RUSTFLAGS").unwrap_or_default();
    let new_rustflags = format!("{} -C target-cpu=native -C target-feature=+aes,+sse2,+sse4.1,+ssse3", current_rustflags);

    println!("cargo:rustc-env=RUSTFLAGS={}", new_rustflags);
    println!("cargo:rerun-if-changed=../.env");

    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.compile().unwrap();
    }
}
