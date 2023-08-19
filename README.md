<p align=center><img src="https://raw.githubusercontent.com/arqalite/rummy-nights/main/public/logo_192.png"/></p>

# <p align=center>Rummy Nights</p> 

Rummy Nights is a rummy score counter PWA (progressive web app) written with [Rust], [Dioxus] and [Tailwind CSS].

Centered around Romanian Tile Rummy ([rules here]), it helps players keep track of scores, bonuses and dealers.

## Try the app
The latest stable release is automatically built and uploaded to Vercel at https://rummy-nights.vercel.app/.

The latest commits are built and uploaded at https://rummy-nights-arqalite.vercel.app/

The Android version is available [on Google Play](https://play.google.com/store/apps/details?id=com.arqalite.rummynights).

The app is available in English and Romanian (the language can be changed in the settings).

## Building from source
In order to build the app, you need:
- [Rust](https://www.rust-lang.org/)
- Rust WASM target (run `rustup target add wasm32-unknown-unknown`)
- Install [Tailwind CSS](https://tailwindcss.com/):
    - We prefer the [standalone CLI](https://github.com/tailwindlabs/tailwindcss/releases/latest). Download it and place it in your PATH as `./tailwindcss`.
    - You can also install it via npm: 
        ```
        npm install -D tailwindcss
        npx tailwindcss init
        ```
- Install [mkcert](https://github.com/FiloSottile/mkcert#installation) otherwise HTTPS will not work.
- Then install [Dioxus CLI](https://github.com/DioxusLabs/dioxus/tree/master/packages/cli): `cargo install dioxus-cli`.

Once set-up, run `dx serve`, click on the IP address displayed on the screen, and it should be good to go!

## Contributing
Pull requests are accepted and encouraged - just go for it!

## Credits
Made with [Rust], [Dioxus], and [Tailwind CSS].

Favicon derived from [this icon, created by Freepik at Flaticon].

All other icons are made by [Ikonate] and [Charm Icons].

Language flags from [Lipis's flag-icons].

Special thanks to the [Trunk](https://trunkrs.dev/) team - we used Trunk until the Dioxus CLI was in usable shape.

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
[1]: https://github.com/DioxusPluginCommunity/tailwind-plugin
