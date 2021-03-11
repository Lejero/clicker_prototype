use druid::*;

use crate::game::character::Character;
use crate::game::GameMessage;
use crate::game::{resource::Resource, resources::Resources};
use std::convert::TryInto;
use std::fmt;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use std::vec::*;

fn always_same() -> bool {
    true
}

#[derive(Clone, Lens, Data)]
pub struct GameState {
    pub resources: Resources,
    pub main_character: Character,
    #[data(ignore)]
    pub msg_tx: mpsc::Sender<GameMessage>,
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.resources == other.resources && self.main_character == other.main_character
    }
}
