fn main() {
    let format = time::format_description::parse_borrowed::<2>("[year][month][day][hour][minute]").unwrap();
    let now = time::OffsetDateTime::now_utc().format(&format).unwrap();
    println!("cargo:rustc-env=VERSION={now}");
}
