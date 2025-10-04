fn main() {
    use ::std::env::var;
    dotenvy::from_path("../.env").ok();
    println!(
        "cargo:rustc-env=APP_NAME={}",
        var("APP_NAME").expect("APP_NAME not found in /.env")
    );
    println!(
        "cargo:rustc-env=APP_TITLE={}",
        var("APP_TITLE").expect("APP_TITLE not found in /.env")
    );
    println!("cargo:rerun-if-changed=../.env");

    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.compile().unwrap();
    }
}
