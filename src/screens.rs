mod game;
mod game_end;
mod menu;
mod player_select;

use dioxus::prelude::*;

pub fn render_menu_screen(cx: Scope) -> Element {
    cx.render(rsx!(menu::screen()))
}

pub fn render_player_select_screen(cx: Scope) -> Element {
    cx.render(rsx!(player_select::screen()))
}

pub fn render_game_screen(cx: Scope) -> Element {
    cx.render(rsx!(game::screen()))
}

pub fn render_game_end_screen(cx: Scope) -> Element {
    cx.render(rsx!(game_end::screen()))
}
