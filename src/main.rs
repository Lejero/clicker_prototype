#![allow(dead_code)]
#![allow(unused_imports)]

use druid::im::{vector, Vector};
use druid::widget::{CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, Padding as Pad, TextBox};
use druid::*;

extern crate clicker_prototype as cp;
use cp::game_loop::game_loop;
use cp::game_loop::game_loop::{GameState, Resource, Resources};
use cp::ui::ui;
use cp::ui::TerminateOnCloseDelegate;

use std::fmt::Debug;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};
use std::vec::*;

const TICKS_PER_SECOND: u64 = 1;

// const UPDATE_RESOURCE: Selector<Resource> = Selector::new("clicker_prototype.update_resource");
// const UPDATE_RESOURCES: Selector<Resource> = Selector::new("clicker_prototype.update_resources");

fn main() -> () {
    let state = init_state();

    let other_state = state.clone();

    //let game_thread = thread::spawn(|| game_loop::start(state2, TICKS_PER_SECOND));

    //let ui_thread = thread::spawn(|| ui::start(state));

    //game_thread.join().expect("game_thread panicked!");
    //let res = ui_thread.join().expect("ui_thread panicked!");

    let window = WindowDesc::new(ui::build_ui).window_size((1000.0, 600.0)).title("Clicker Prototype");
    let launcher = AppLauncher::with_window(window).delegate(TerminateOnCloseDelegate {});

    let ui_handle = launcher.get_external_handle();
    let game_thread = thread::spawn(|| game_loop::start(other_state, ui_handle, TICKS_PER_SECOND));

    launcher.launch(state);
}

fn init_state() -> game_loop::GameState {
    GameState {
        resources: Resources {
            iron: Resource {
                name: String::from("Iron"),
                quantity: 5,
                user_collectable: true,
            },
            copper: Resource {
                name: String::from("Copper"),
                quantity: 1,
                user_collectable: true,
            },
            tin: Resource {
                name: String::from("Tin"),
                quantity: 0,
                user_collectable: true,
            },
            nickel: Resource {
                name: String::from("Nickel"),
                quantity: 0,
                user_collectable: true,
            },
            coal: Resource {
                name: String::from("Coal"),
                quantity: 0,
                user_collectable: true,
            },
            steel: Resource {
                name: String::from("Steel"),
                quantity: 0,
                user_collectable: true,
            },
        },
    }
}
