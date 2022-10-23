use chrono::prelude::*;
use std::env;

fn main() {
    let sys_time = Local::now();
    let formatted_time = format!("{}", sys_time.format("%Y-%m-%d %H:%M:%S"));
    let mut version = String::new();

    if let Ok(profile) = env::var("PROFILE") {
        if profile == "release" {
            version = "v".to_owned() + &env::var("CARGO_PKG_VERSION").unwrap();
        }
        if profile == "debug" {
            version = formatted_time;
        }
    }

    println!("cargo:rustc-env=BUILD_VERSION={}", version);
}
