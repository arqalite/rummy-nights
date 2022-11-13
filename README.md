# Rummy Nights
Rummy Nights is a rummy score counter PWA (progressive web app) written with [Rust], [Dioxus] and [Tailwind CSS].

It aims to be a helpful tool for players of Romanian Tile Rummy ([rules here]), helping them keep track of scores, bonuses and dealers, all in a lightweight package that runs on both desktop and mobile.

## Project status
Currently the app is undergoing heavy development. Expect things to change wildly and quickly.

There are rough edges, bugs and unfinished features, but versioned releases should be usable and relatively stable.

## Try the app
The latest stable release is automatically built and uploaded to Vercel at https://rummy-nights.vercel.app/.

The latest commits are built and uploaded at https://rummy-nights-arqalite.vercel.app/

## Building from source
In order to build the app, you need:
- [Rust](https://www.rust-lang.org/)
- Rust WASM target (run `rustup target add wasm32-unknown-unknown`)
- [Trunk](https://trunkrs.dev/) (you can get it with `cargo install trunk`)
- [Tailwind CSS](https://tailwindcss.com/) - make sure to have the [standalone CLI executable](https://tailwindcss.com/blog/standalone-cli) in your PATH (Trunk needs it to generate the CSS on-the-fly)

Once set-up, run `trunk serve --open` and it should be good to go!

## Contributing
Pull requests are accepted and encouraged.
Before contributing, please read [CONTRIBUTING.md](./CONTRIBUTING.md) for more details and instructions.

Our contributors:

<a href="https://github.com/arqalite/rummy-nights/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=arqalite/rummy-nights" />
</a>

<sub>Made with [contrib.rocks](https://contrib.rocks)</sub>

## Credits
Made with [Rust], [Dioxus] and [Tailwind CSS].
Icons by [Ikonate] and [Charm Icons].

## License
This project is licensed under the [MIT license](https://github.com/arqalite/rummy-nights/blob/main/LICENSE).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Rummy Nights, shall be licensed as MIT, without any additional
terms or conditions. We reserve the right to reject contributions that will not be licensed as such.

[Rust]: https://www.rust-lang.org/
[Dioxus]: https://dioxuslabs.com/
[Tailwind CSS]: https://tailwindcss.com/
[Ikonate]: https://ikonate.com/
[Charm Icons]: https://github.com/jaynewey/charm-icons
[rules here]: https://www.pagat.com/rummy/romtile.html