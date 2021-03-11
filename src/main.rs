#![allow(dead_code)]
#![allow(unused_imports)]

use druid::im::{vector, Vector};
use druid::widget::{CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, Padding as Pad, TextBox};
use druid::*;

extern crate clicker_prototype as cp;
use cp::game;
use cp::game::character::Character;
use cp::game::{
    resource::Resource,
    resources::{Resources, HMT, OMT},
};
use cp::game::{GameMessage, GameState};
use cp::ui::ui;
use cp::ui::TerminateOnCloseDelegate;

use std::fmt::Debug;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use std::vec::*;

const TICKS_PER_SECOND: u64 = 20;

// const UPDATE_RESOURCE: Selector<Resource> = Selector::new("clicker_prototype.update_resource");
// const UPDATE_RESOURCES: Selector<Resource> = Selector::new("clicker_prototype.update_resources");

fn main() -> Result<(), PlatformError> {
    let (tx, rx) = mpsc::channel::<GameMessage>();

    let state = init_state(tx);
    let other_state = state.clone();

    //let game_thread = thread::spawn(|| game_loop::start(state2, TICKS_PER_SECOND));

    //let ui_thread = thread::spawn(|| ui::start(state));

    //game_thread.join().expect("game_thread panicked!");
    //let res = ui_thread.join().expect("ui_thread panicked!");

    let window = WindowDesc::new(ui::build_ui)
        .window_size(ui::MIN_RESOLUTION)
        .title("Clicker Prototype");
    let launcher = AppLauncher::with_window(window).delegate(TerminateOnCloseDelegate {});

    let ui_handle = launcher.get_external_handle();
    let _game_thread = thread::spawn(|| game::start(other_state, rx, ui_handle, TICKS_PER_SECOND));

    launcher.launch(state)
}

fn init_state(msg_tx: mpsc::Sender<GameMessage>) -> GameState {
    let mut resource_ls = im::OrdMap::<String, Resource>::new();
    resource_ls.insert(String::from("Iron"), Resource::new("Iron", 5));
    resource_ls.insert(String::from("Copper"), Resource::new("Copper", 1));
    resource_ls.insert(String::from("Tin"), Resource::new("Tin", 0));
    resource_ls.insert(String::from("Nickel"), Resource::new("Nickel", 0));
    resource_ls.insert(String::from("Coal"), Resource::new("Coal", 0));
    resource_ls.insert(String::from("Steel"), Resource::new("Steel", 0));

    // let mut resource_ls = im::Vector::<Resource>::new();
    // resource_ls.push_back(Resource::new("Iron", 5));
    // resource_ls.push_back(Resource::new("Copper", 1));
    // resource_ls.push_back(Resource::new("Tin", 0));
    // resource_ls.push_back(Resource::new("Nickel", 0));
    // resource_ls.push_back(Resource::new("Coal", 0));
    // resource_ls.push_back(Resource::new("Steel", 0));

    GameState {
        resources: Resources {
            ls: OMT { m: resource_ls },
            // iron: Resource::new("Iron", 5),
            // copper: Resource::new("Copper", 1),
            // tin: Resource::new("Tin", 0),
            // nickel: Resource::new("Nickel", 0),
            // coal: Resource::new("Coal", 0),
            // steel: Resource::new("Steel", 0),
        },
        main_character: Character::new("Lohse"),
        msg_tx: msg_tx,
    }
}
