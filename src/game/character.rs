use druid::*;

use crate::ui::events::{UPDATE_RESOURCE, UPDATE_RESOURCES};
use std::convert::TryInto;
use std::fmt;
// use std::fmt::{Debug, Display};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::vec::*;

#[derive(Clone, Lens, Data)]
pub struct Character {
    //We can send events to the UI here. Events the UI expects are currently defined in that module.
    #[data(ignore)]
    ui_event_hdl: Arc<druid::ExtEventSink>,
    pub name: String,
    pub health: f64,
    pub mana: f64,
    pub resource_collecting: Option<String>,
    pub work_power_sec: u64,
    pub work_accumulated: Option<u64>,
}

impl Character {
    pub fn new(ui_event_hdl: Arc<druid::ExtEventSink>, name: &str) -> Self {
        Character {
            ui_event_hdl,
            name: String::from(name),
            health: 100.0,
            mana: 100.0,
            resource_collecting: None,
            work_power_sec: 10,
            work_accumulated: None,
        }
    }
    pub fn update(&mut self) {}
    pub fn msg_update(&mut self) {}
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0}: {1}, {2}", self.name, self.health, self.mana)
    }
}

impl PartialEq for Character {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.health == other.health && self.mana == other.mana
    }
}
