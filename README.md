<img src="https://raw.githubusercontent.com/arqalite/rummy-nights/main/assets/img/logo_192.png" />

# Rummy Nights
Rummy Nights is a rummy score counter PWA (progressive web app) written with [Rust], [Dioxus] and [Tailwind CSS].

It aims to be a helpful tool for players of Romanian Tile Rummy ([rules here]), helping them keep track of scores, bonuses and dealers, all in a lightweight package that runs on both desktop and mobile.

## Project status
The project is still in development - but as of v0.3, the core functionality is complete.

The app is fairly stable, but bugs can still crop up.

The interface is mobile-first, with the desktop experience being left to the side at the moment. 
It works fine, but UI elements might look wonky on HIDPI displays.

## Try the app
The latest stable release is automatically built and uploaded to Vercel at https://rummy-nights.vercel.app/.

The latest commits are built and uploaded at https://rummy-nights-arqalite.vercel.app/

The app is available in English and Romanian (the language can be changed in the settings).

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

Favicon derived from [this icon, created by Freepik at Flaticon].

All other icons are made by [Ikonate] and [Charm Icons].

Language flags from [Lipis's flag-icons].

## License
This project is licensed under the [MIT license](https://github.com/arqalite/rummy-nights/blob/main/LICENSE).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Rummy Nights, shall be licensed as MIT, without any additional
terms or conditions. We reserve the right to reject contributions that will not be licensed as such.

[Rust]: https://www.rust-lang.org/
[Dioxus]: https://dioxuslabs.com/
[Tailwind CSS]: https://tailwindcss.com/
[this icon, created by Freepik at Flaticon]: https://www.flaticon.com/free-icon/poker_8304852?term=gambling&page=1&position=20&page=1&position=20&related_id=8304852&origin=style
[Ikonate]: https://ikonate.com/
[Charm Icons]: https://github.com/jaynewey/charm-icons
[Lipis's flag-icons]: https://github.com/lipis/flag-icons
[rules here]: https://www.pagat.com/rummy/romtile.html