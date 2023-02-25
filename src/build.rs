use chrono::prelude::*;
use std::env;

fn main() {
    let mut version = String::new();

    if let Ok(profile) = env::var("PROFILE") {
        if profile == "release" {
            version = "v".to_owned() + &env::var("CARGO_PKG_VERSION").unwrap();
        }
        if profile == "debug" {
            version = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        }
    }

    println!("cargo:rustc-env=BUILD_VERSION={version}");
}
