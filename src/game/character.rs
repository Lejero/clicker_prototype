use druid::*;

use crate::ui::events::{UPDATE_RESOURCE, UPDATE_RESOURCES};
use std::convert::TryInto;
use std::fmt;
// use std::fmt::{Debug, Display};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};
use std::vec::*;

#[derive(Clone, Lens, Data, Debug, PartialEq)]
pub struct Character {
    pub name: String,
    pub health: f64,
    pub mana: f64,
}

impl Character {
    pub fn new(name: &str) -> Self {
        Character {
            name: String::from(name),
            health: 100.0,
            mana: 100.0,
        }
    }
}
