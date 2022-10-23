use chrono::prelude::*;

fn main() {
    let sys_time = Local::now();
    let formatted_time = format!("{}", sys_time.format("%Y-%m-%d %H:%M:%S"));

    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", formatted_time);
}