use dioxus::prelude::*;
use dioxus_web::use_eval;
use gloo_storage::{LocalStorage, SessionStorage, Storage};
use rummy_nights::prelude::*;
use std::cmp::Ordering;

pub fn main() {
    log!("Initializing app.");

    dioxus_web::launch(|cx| {
        fermi::use_init_atom_root(cx);
        let state = fermi::use_atom_ref(cx, STATE);
        let lang_code = state.read().settings.language;
        let sorted_players = state.read().game.sorted_players.clone();

        log!("Loaded new state.");

        if !state.read().checked_storage {
            state.write().load_existing_game();
            state.write().settings.load();
            state.write().load_saved_templates();
            log!("Finish loading data.");
        };

        let delete_and_exit_game = move |_| {
            log!("Deleting game and returning to main menu.");
            LocalStorage::delete("state");
            SessionStorage::delete("session");
            *state.write() = Model::new();
        };

        let restart_game = move |_| {
            log!("Restarting game.");
            state.write().reset_game();
        };

        if state.read().screen == Screen::EndGame && !state.read().game.is_sorted {
            state.write().game.sort_players();
        }

        let on_activate_tile_bonus = move |_| {
            if state.read().game.tile_bonus_button_active {
                state.write().game.tile_bonus_button_active = false;
            } else if !state.read().game.tile_bonus_granted
                && state.read().game.status == GameStatus::Ongoing
            {
                state.write().game.tile_bonus_button_active = true;
            };
        };

        let on_score_input = move |(evt, player_id, len)| {
            let evt: FormEvent = evt;
            let len: usize = len;

            if let Ok(score) = evt.values.get("score").unwrap().parse::<i32>() {
                state.write().add_score(player_id, score);
            };

            let focus_id = match player_id.cmp(&len) {
                Ordering::Greater => 5,
                Ordering::Equal => 1,
                Ordering::Less => player_id + 1,
            };
            use_eval(cx)(format!(
                "document.getElementById('{player_id}').value = '';"
            ));
            use_eval(cx)(format!("document.getElementById('{focus_id}').focus();"));
        };

        let on_grant_tile_bonus = move |(_, player_id)| {
            if !state.read().game.tile_bonus_granted && state.read().settings.use_tile_bonus {
                state.write().game.grant_bonus(player_id);
            }
        };

        let on_score_edit = move |evt| {
            let evt: Event<FormData> = evt;
            log!(format!("This has {:?}", evt.values));
            if let Ok(score) = evt.values.get("score").unwrap().parse::<i32>() {
                if let Ok(score_id) = evt.values.get("score_id").unwrap().parse::<usize>() {
                    if let Ok(player_id) = evt.values.get("player_id").unwrap().parse::<usize>() {
                        state.write().edit_score(player_id, score_id, score);
                    }
                }
            };
        };

        log!("Start render.");
        render!(
            div {
                class: "flex flex-col bg-white h-screen w-screen relative overflow-hidden",
                div {
                    class: "z-10 flex flex-col h-screen mx-auto w-full sm:max-w-lg",
                    match state.read().screen {
                        Screen::Menu => rsx!(rummy_nights::frontend::menu::MenuScreen {
                            lang_code: state.read().settings.language,
                            game_status: state.read().game.status,
                            on_click_settings: move |_| state.write().screen = Screen::Settings,
                            on_click_start:  move |_| state.write().create_game(),
                            on_click_resume: move |_| state.write().screen = Screen::Game,

                        }),
                        Screen::PlayerSelect => rsx!(rummy_nights::frontend::player_select::PlayerSelectScreen {
                            lang_code: lang_code,
                            game: state.read().game.clone(),
                            on_click_begin: move |_| state.write().start_game(),
                            on_click_back: move |_| {
                                    state.write().screen = Screen::Menu;
                                    state.write().checked_storage = false;
                                    SessionStorage::delete("session");
                                },
                            on_click_template: move |_| state.write().screen = Screen::Templates,
                            on_add_player: move |(evt, color_index)| {
                                let evt: FormEvent = evt;
                                let color_index: usize = color_index;
                                let name = evt.values.get("player-name").unwrap().to_string();

                                if !name.is_empty() {
                                    state.write().game.add_player(name, color_index);

                                    //Execute some JS on the spot - weird ergonomics but it works
                                    use_eval(cx)(String::from(
                                        "document.getElementById('name_input').reset();",
                                    ));
                                };
                            },
                            on_edit_player: move |(evt,id)| {
                                let evt: FormEvent = evt;
                                let id: usize = id;
                                let name = evt.values.get("player-name").unwrap().to_string();
                                if !name.is_empty() {
                                    state.write().game.edit_player_name(id - 1, name);
                                };
                            },
                            on_remove_player: move |(_, id)| state.write().game.remove_player(id),
                            on_move_up: move |(_, id)| state.write().game.move_up(id),
                            on_move_down: move |(_, id)| state.write().game.move_down(id),
                            on_color_change: move |(_, id, color_id)| {
                                state.write().game.change_player_color(id, color_id);
                            }

                        }),
                        Screen::Templates => rsx!(rummy_nights::frontend::templates::TemplateScreen {
                            lang_code: lang_code,
                            game: state.read().game.clone(),
                            templates: state.read().templates.clone(),
                            on_add_template: move |_| state.write().add_template(),
                            on_edit_template: move |(evt, color_index)| {
                                let evt: FormEvent = evt;
                                let color_index: usize = color_index;
                                let name = evt.values.get("template-name").unwrap().to_string();
                                if !name.is_empty() {
                                    if let Ok(template_id) = evt.values.get("template_id").unwrap().parse::<usize>() {
                                        state.write().edit_template(template_id, name, color_index);
                                    }
                                };
                            },
                            on_delete_template: move |(_, id)| state.write().delete_template(id),
                            on_load_template: move |(_, id)| state.write().load_template(id),
                            on_return_to_select: move |_| {
                                state.write().screen = Screen::PlayerSelect;
                            },
                        }),
                        Screen::Game => rsx!(rummy_nights::frontend::game::GameScreen {
                            lang_code: lang_code,
                            game: state.read().game.clone(),
                            use_tile_bonus: state.read().settings.use_tile_bonus,
                            tile_bonus_button_active: state.read().game.tile_bonus_button_active,
                            tile_bonus_granted: state.read().game.tile_bonus_granted,
                            enable_dealer_tracking: state.read().settings.enable_dealer_tracking,
                            enable_score_editing: state.read().settings.enable_score_editing,
                            on_click_playerselect: move |_| state.write().screen = Screen::PlayerSelect,
                            on_click_home: move |_| state.write().screen = Screen::Menu,
                            on_click_end: move |_| state.write().screen = Screen::EndGame,
                            on_score_edit: on_score_edit,
                            on_score_input: on_score_input,
                            on_click_player: on_grant_tile_bonus,
                            on_click_tile_bonus: on_activate_tile_bonus
                        }),
                        Screen::EndGame => {
                            rsx!(
                                rummy_nights::frontend::game_end::EndScreen {
                                    players: sorted_players,
                                    lang_code: lang_code,
                                    on_click_home: delete_and_exit_game,
                                    on_click_restart: restart_game,
                                    on_click_back: move |_| state.write().screen = Screen::Game,

                                }
                            )
                        },
                        Screen::Settings => rsx!(rummy_nights::frontend::settings::SettingsScreen {
                            settings: state.read().settings,
                            on_restart_app: move |_| {
                                SessionStorage::clear();
                                use_eval(cx)("location.reload()");
                            },
                            on_clear_data: move |_| {
                                LocalStorage::clear();
                                SessionStorage::clear();
                                use_eval(cx)("location.reload()");
                            },
                            on_tile_enable: move |enabled: bool| {
                                state.write().settings.use_tile_bonus = enabled;
                                log!(format!("Tile bonus is {:?}", state.read().settings.use_tile_bonus));
                            },
                            on_set_tile_bonus_value: move |value: i32| {
                                log!(format!("setting tile bonus to {value}"));
                                state.write().settings.set_tile_bonus(value);
                                use_eval(cx)(format!(
                                    "document.getElementById('max_score').value = '{value}';"
                                ));
                            },
                            on_set_language: |value: usize| state.write().settings.language = value,
                            on_return_to_menu: move |_| {
                                state.write().settings.save();
                                state.write().screen = Screen::Menu;
                            },
                            on_go_to_credits: move |_| {
                                state.write().settings.save();
                                state.write().screen = Screen::Credits;
                            },
                            on_toggle_edit: move |value: bool| {
                                state.write().settings.enable_score_editing = value;
                                log!(format!("Score editing enabled: {:?}", state.read().settings.enable_score_editing));
                            }
                            on_toggle_dealer: move |value: bool| {
                                state.write().settings.enable_dealer_tracking = value;
                                log!(format!("Dealer enabled: {:?}", state.read().settings.enable_dealer_tracking));
                            }
                            on_toggle_max_score: move |enabled: bool| {
                                state.write().settings.end_game_at_score = enabled;
                                log!(format!("Max score enabled: {:?}", state.read().settings.end_game_at_score));
                            },
                            on_score_change: move |value: i32| {
                                log!(format!("setting score to {value}"));
                                state.write().settings.set_max_score(value);
                                use_eval(cx)(format!(
                                    "document.getElementById('max_score').value = '{value}';"
                                ));
                            }
                        }),
                        Screen::Credits => rsx!(rummy_nights::frontend::credits::CreditsScreen {
                            on_click: |_| state.write().screen = Screen::Settings,
                            lang_code: lang_code,
                        }),
                    },
                }
                rummy_nights::frontend::DecorativeSpheres {
                    screen: state.read().screen
                }
            }
        )
    });
}
