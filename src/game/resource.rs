use druid::im::HashMap;
use druid::widget::ListIter;
use druid::*;

use crate::game::{GameMessage, GameState};
use crate::ui::events::{UPDATE_RESOURCE, UPDATE_RESOURCES};
use std::cmp::Ordering;
use std::convert::TryInto;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::vec::*;

#[derive(Clone, Lens, Data)]
pub struct Resource {
    //We can send events to the UI here. Events the UI expects are currently defined in that module.
    #[data(ignore)]
    pub ui_event_hdl: Arc<druid::ExtEventSink>,
    pub name: String,
    pub quantity: i64,
    pub player_collectable: bool,
    pub player_visible: bool,
    pub collection_work: u64,
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0}: {1}", self.name, self.quantity)
    }
}

impl PartialEq for Resource {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.quantity == other.quantity
    }
}

impl Resource {
    pub fn new(ui_event_hdl: Arc<druid::ExtEventSink>, name: &str, qty: i64) -> Self {
        Resource {
            ui_event_hdl,
            name: String::from(name),
            quantity: qty,
            player_collectable: true,
            player_visible: true,
            collection_work: 10,
        }
    }

    pub fn add_qty(&mut self, qty: i64) {
        self.quantity += qty;

        self.msg_update();
    }
    pub fn msg_update(&mut self) {
        if self
            .ui_event_hdl
            .submit_command(UPDATE_RESOURCE, self.clone(), Target::Auto)
            .is_err()
        {
            println!("Druid Command Error - Updating Resource '{}'", self.name);
        }
    }
}
