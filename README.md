# Rummy Nights
A rummy score counter web app written with Rust/Dioxus and Tailwind CSS.

## Try the app
The latest commits are automatically built and uploaded to Vercel at https://rummy-nights.vercel.app/.

## Building from source
In order to build the app, you need:
- [Rust](https://www.rust-lang.org/)
- Rust WASM target (run `rustup target add wasm32-unknown-unknown`)
- [Trunk](https://trunkrs.dev/) (you can get it with `cargo install trunk`)
- [Tailwind CSS](https://tailwindcss.com/) - make sure to have the [standalone CLI executable](https://tailwindcss.com/blog/standalone-cli) in your PATH (Trunk needs it to generate the CSS on-the-fly)

Once set-up, run `trunk serve --open` and it should be good to go.
