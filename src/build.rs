use std::env;
use chrono::prelude::*;

fn main() {
    let sys_time = Local::now();
    let formatted_time = format!("{}", sys_time.format("%Y-%m-%d %H:%M:%S"));
    let mut version = String::new();

    match env::var("PROFILE") {
        Ok(profile) => {
            if profile == "release" {
                version = "v".to_owned() + &env::var("CARGO_PKG_VERSION").unwrap();
            }
            if profile == "debug" {
                version = formatted_time;
            }
        },
        Err(_) => ()
    }

    println!("cargo:rustc-env=BUILD_VERSION={}", version);

}