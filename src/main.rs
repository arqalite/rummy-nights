use dioxus::prelude::*;

mod statics;
mod score_table;
mod game_menu;

static PLAYERS: Atom<Vec<Player>> = |_| vec![
        Player {
            id: 1,
            name: "Antonio".to_string(),
            score: vec![],
        },
        Player {
            id: 2,
            name: "Vlad".to_string(),
            score: vec![],
        },
        Player {
            id: 3,
            name: "Dalmina".to_string(),
            score: vec![],
        },
        Player {
            id: 4,
            name: "Daniel".to_string(),
            score: vec![],
        }
];

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        score_table::score_table(),
        div {
            crate::game_menu::game_menu()
        }
    ))
}

#[derive(PartialEq, Clone)]
struct Player {
    id: usize,
    name: String,
    score: Vec<i32>
}