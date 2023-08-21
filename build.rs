fn main() {
    let now = chrono::Utc::now().format("%Y%m%d%H%M");
    println!("cargo:rustc-env=VERSION={now}");
}
