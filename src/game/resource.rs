use druid::im::HashMap;
use druid::widget::ListIter;
use druid::*;

use crate::ui::events::{UPDATE_RESOURCE, UPDATE_RESOURCES};
use std::convert::TryInto;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};
use std::vec::*;

#[derive(Clone, Lens, Data, Debug, PartialEq)]
pub struct Resource {
    pub name: String,
    pub quantity: i64,
    pub player_collectable: bool,
    pub player_visible: bool,
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0}: {1}", self.name, self.quantity)
    }
}

impl Resource {
    pub fn new(name: &str, qty: i64) -> Self {
        Resource {
            name: String::from(name),
            quantity: qty,
            player_collectable: true,
            player_visible: true,
        }
    }

    pub fn add_qty(&mut self, qty: i64) {
        self.quantity += qty;
    }
}
