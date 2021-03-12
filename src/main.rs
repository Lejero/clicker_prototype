#![allow(dead_code)]
#![allow(unused_imports)]

use druid::im::{vector, Vector};
use druid::widget::{CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, Padding as Pad, TextBox};
use druid::*;

extern crate clicker_prototype as cp;
use cp::game;
use cp::game::{character::Character, resource::Resource, resources::Resources};
use cp::game::{GameMessage, GameState};
use cp::ui::ui;
use cp::ui::TerminateOnCloseDelegate;

use std::fmt::Debug;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use std::vec::*;

const TICKS_PER_SECOND: u64 = 20;

// const UPDATE_RESOURCE: Selector<Resource> = Selector::new("clicker_prototype.update_resource");
// const UPDATE_RESOURCES: Selector<Resource> = Selector::new("clicker_prototype.update_resources");

fn main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(ui::build_ui())
        .window_size(ui::MIN_RESOLUTION)
        .title("Clicker Prototype");
    let launcher = AppLauncher::with_window(window).delegate(TerminateOnCloseDelegate {});
    let (tx, rx) = mpsc::channel::<GameMessage>();
    let ui_handle = launcher.get_external_handle();
    let state = init_state(ui_handle, tx);
    let other_state = state.clone();

    let ui_handle = launcher.get_external_handle();
    let _game_thread = thread::spawn(|| game::start(other_state, rx, ui_handle, TICKS_PER_SECOND));

    launcher.launch(state)
}

fn init_state(ui_event_hdl: ExtEventSink, msg_tx: mpsc::Sender<GameMessage>) -> GameState {
    let mut resource_ls = im::OrdMap::<String, Resource>::new();
    let hdl = Arc::new(ui_event_hdl);
    resource_ls.insert(
        String::from("Iron"),
        Resource {
            collection_work: 20,
            ..Resource::new(hdl.clone(), "Iron", 5)
        },
    );
    resource_ls.insert(String::from("Copper"), Resource::new(hdl.clone(), "Copper", 1));
    resource_ls.insert(
        String::from("Wood"),
        Resource {
            collection_work: 5,
            ..Resource::new(hdl.clone(), "Wood", 0)
        },
    );
    resource_ls.insert(String::from("Stone"), Resource::new(hdl.clone(), "Stone", 0));
    resource_ls.insert(String::from("Coal"), Resource::new(hdl.clone(), "Coal", 0));
    resource_ls.insert(String::from("Steel"), Resource::new(hdl.clone(), "Steel", 0));

    // let mut resource_ls = im::Vector::<Resource>::new();
    // resource_ls.push_back(Resource::new("Iron", 5));
    // resource_ls.push_back(Resource::new("Copper", 1));
    // resource_ls.push_back(Resource::new("Tin", 0));
    // resource_ls.push_back(Resource::new("Nickel", 0));
    // resource_ls.push_back(Resource::new("Coal", 0));
    // resource_ls.push_back(Resource::new("Steel", 0));

    GameState {
        resources: Resources { ls: resource_ls },
        main_character: Character::new(hdl.clone(), "Lohse"),
        msg_tx: msg_tx,
    }
}
