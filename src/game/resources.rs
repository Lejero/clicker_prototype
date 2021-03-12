use druid::im::{HashMap, OrdMap};
use druid::widget::ListIter;
use druid::*;

use crate::game::resource::Resource;
use crate::ui::events::{UPDATE_RESOURCE, UPDATE_RESOURCES};
use std::convert::TryInto;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};
use std::vec::*;

pub enum ResourceNames {
    IronOre,
    CopperOre,
    TinOre,
    NickelOre,
    Coal,
    IronPlate,
    CopperPlate,
    TinPlate,
    NickelPlate,
    Steel,
    Other(String),
}

#[derive(Clone, Lens, Data, PartialEq)]
pub struct Resources {
    pub ls: im::OrdMap<String, Resource>,
}

impl Resources {
    pub fn new() -> Self {
        Resources { ls: im::OrdMap::new() }
    }

    pub fn add_resource(&mut self, res: Resource) {
        self.ls.insert(res.name.clone(), res);
    }
}
