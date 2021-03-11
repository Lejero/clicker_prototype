use crate::game::GameState;

use std::sync::mpsc;

pub enum GameMessage {
    MineResource(String),
}

impl GameMessage {
    pub fn msg_mine_resource(msg_tx: mpsc::Sender<GameMessage>, s: impl Into<String>) {
        msg_tx
            .send(GameMessage::MineResource(s.into()))
            .expect("Message to game loop from ui failed to send");
    }
}
