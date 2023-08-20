#!/bin/sh

curl https://sh.rustup.rs -sSf | sh -s -- -y
source "$HOME/.cargo/env"
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli
/vercel/.cargo/bin/dx build --release