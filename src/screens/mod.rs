//! The front-end part of the app, rendering the individual app screens.

mod game;
mod game_end;
mod menu;
mod player_select;

use crate::data::Screen;
use dioxus::prelude::*;

pub fn render_screen<'a>(cx: Scope<'a>, screen: &Screen) -> Element<'a> {
    cx.render(rsx!(
        div {
            class: "bg-white",
            match *screen {
                Screen::Menu => render_menu_screen(cx),
                Screen::PlayerSelect => render_player_select_screen(cx),
                Screen::Game => render_game_screen(cx),
                Screen::Winner => render_game_end_screen(cx),
            }
        }
    ))
}

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
