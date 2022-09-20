# Rummy Nights
A rummy score counter web app written with Rust/Dioxus and Tailwind CSS.

Currently in alpha stage, but 1.0 is coming soon. :)

## Try the app
The latest commits are automatically built and uploaded to Vercel at https://rummy-nights.vercel.app/.

Once we reach a stable version, we'll have a separate server for releases.

## Building from source
In order to build the app, you need:
- [Rust](https://www.rust-lang.org/)
- [Trunk](https://trunkrs.dev/) (you can get it with `cargo install trunk`)
- [Tailwind CSS](https://tailwindcss.com/) - make sure to have the [standalone CLI executable](https://tailwindcss.com/blog/standalone-cli) in your PATH (Trunk needs it to generate the CSS on-the-fly)

Once set-up, run `trunk serve --open` and it should be good to go.

## Contributing
PRs are welcome, however keep in mind this is a personal project and design decisions are made based on my and my friends' preferences.
