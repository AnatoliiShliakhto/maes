fn main() {
    use ::std::env::var;
    dotenvy::from_path("../.env").ok();


    // #[cfg(all(windows, target_env = "msvc"))]
    // {
    //     let current_rustflags = var("RUSTFLAGS").unwrap_or_default();
    //     let rustflags = format!("{current_rustflags} -C target-cpu=native -C target-feature=+aes,+sse2,+sse4.1,+ssse3,+crt-static");
    //     println!("cargo:rustc-env=RUSTFLAGS={rustflags}");
    // }

    println!("cargo:rerun-if-changed=../.env");

    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.compile().unwrap();
    }
}
