#![allow(dead_code)]
#![allow(unused_imports)]

use druid::*;

use crate::ui::ui::{UPDATE_RESOURCE, UPDATE_RESOURCES};
use std::convert::TryInto;
use std::fmt;
// use std::fmt::{Debug, Display};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};
use std::vec::*;

pub enum ResourceNames {
    IRON_ORE,
    COPPER_ORE,
    TIN_ORE,
    NICKEL_ORE,
    COAL,
    IRON_PLATE,
    COPPER_PLATE,
    TIN_PLATE,
    NICKEL_PLATE,
    STEEL,
    OTHER(String),
}

#[derive(Clone, Lens, Data)]
pub struct GameState {
    pub resources: Resources,
}

#[derive(Clone, Lens, Data, PartialEq)]
pub struct Resources {
    pub iron: Resource,
    pub copper: Resource,
    pub tin: Resource,
    pub nickel: Resource,
    pub coal: Resource,
    pub steel: Resource,
}

#[derive(Clone, Lens, Data, Debug, PartialEq)]
pub struct Resource {
    pub name: String,
    pub quantity: i64,
    pub user_collectable: bool,
}
#[derive(Clone, Lens, Data, Debug)]
pub struct Character {}

impl fmt::Display for Resource {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{0}: {1}", self.name, self.quantity)
    }
}

pub fn start(mut init_state: GameState, ui_handle: druid::ExtEventSink, ticks_per_second: u64) {
    let wait_time = Duration::from_millis(1000 / ticks_per_second);

    let mut counter = 0;

    // loop {}

    loop {
        println!("loop {}", counter);
        thread::sleep(wait_time);

        init_state.resources.iron.quantity += 1;

        if ui_handle.submit_command(UPDATE_RESOURCES, init_state.resources.clone(), Target::Auto).is_err() {
            break;
        }

        counter += 1;
    }
}
