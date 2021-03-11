use druid::*;

use crate::game::character::Character;
use crate::game::resource::{Resource, Resources};
use crate::game::{GameMessage, GameState};
use crate::ui::events::{UPDATE_RESOURCE, UPDATE_RESOURCES};
use std::convert::TryInto;
use std::fmt;
// use std::fmt::{Debug, Display};
use std::cmp::Ordering;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use std::vec::*;

struct StateWrapper {
    //We can send events to the UI here. Events the UI expects are currently defined in that module.
    ui_event_hdl: druid::ExtEventSink,
    //We can look for messages from other threads here wit try_iter()
    msg_rx: mpsc::Receiver<GameMessage>,
    //The game state, independent of handlers, it implements Data and druid uses the same type for the UI.
    state: GameState,
    wait_time: Duration,
}

impl StateWrapper {
    pub fn cycle(&mut self) {
        let mut _counter = 0;

        let mut add_iron_event = TimedTrigger::new(Duration::from_millis(1000));
        let mut add_copper_event = TimedTrigger::new(Duration::from_millis(1800));

        //In the loop we have a few stages
        // 1. We check for events from other threads and handle them if there are any.
        // 2. We update our state that needs to be updated based on a timer.GameState
        // 3. Might not be necessary, but we sleep. I think at the very least I'd like to sleep until a time and not for a time so that the update loop doesn't run doing nothing many times a second, but it'll keep running consistently.
        loop {
            // println!("loop {}", counter);
            //let received = msg_rx.recv().expect("Error with between thread messages of mpsc type.");
            for m in self.msg_rx.try_iter() {
                handle_message(m, &self.ui_event_hdl, &mut self.state);
            }

            add_iron_event.if_elapsed(|| self.msg_update_resource("Iron"));
            add_copper_event.if_elapsed(|| self.msg_update_resource("Copper"));

            _counter += 1;
            thread::sleep(self.wait_time);
        }
    }

    pub fn msg_update_resource(&mut self, name: &str) {
        let res_opt = self.state.resources.ls.m.get_mut(name);

        match res_opt {
            Some(res) => {
                res.quantity += 1;

                if self
                    .ui_event_hdl
                    .submit_command(UPDATE_RESOURCE, res.clone(), Target::Auto)
                    .is_err()
                {
                    println!("Druid Command Error - Updating Resource '{}'", res.name);
                }
            }
            None => {
                println!("druid::im::Vector Error - Finding index of '{}'", "Iron");
            }
        }
    }
    fn handle_message(&mut self, msg: GameMessage) {
        match msg {
            GameMessage::MineResource(resource_name) => self.msg_update_resource(&resource_name),
        }
    }

    pub fn msg_update_resource_alt(&self, res: &Resource) {
        if self
            .ui_event_hdl
            .submit_command(UPDATE_RESOURCE, res.clone(), Target::Auto)
            .is_err()
        {
            println!("Druid Command Error - Updating Resource '{}'", res.name);
        }
    }
}

pub fn start(
    state: GameState,
    msg_rx: mpsc::Receiver<GameMessage>,
    event_handle: druid::ExtEventSink,
    ticks_per_second: u64,
) {
    let mut x = StateWrapper {
        ui_event_hdl: event_handle,
        msg_rx: msg_rx,
        state: state,
        wait_time: Duration::from_millis(1000 / ticks_per_second),
    };

    if x.ui_event_hdl
        .submit_command(UPDATE_RESOURCES, x.state.resources.clone(), Target::Auto)
        .is_err()
    {
        println!("Druid Command Error Updating Resource Copper");
    }

    x.cycle();
}

fn update_resource_gather() {}

fn handle_message(msg: GameMessage, event_handle: &ExtEventSink, state: &mut GameState) {
    match msg {
        GameMessage::MineResource(resource_name) => {
            let res_opt = state.resources.ls.m.get_mut(&resource_name);

            match res_opt {
                Some(res) => {
                    res.quantity += 1;

                    if event_handle
                        .submit_command(UPDATE_RESOURCE, res.clone(), Target::Auto)
                        .is_err()
                    {
                        println!("Druid Command Error - Updating Resource '{}'", res.name);
                    }
                }
                None => {
                    println!("druid::im::Vector Error - Finding index of '{}'", resource_name);
                }
            }
        }
    }
}

struct TimedTrigger {
    pub dur: Duration,
    pub last_time: Instant,
    pub waited_dur: Duration,
}

impl TimedTrigger {
    pub fn new(dur: Duration) -> Self {
        TimedTrigger {
            dur,
            last_time: Instant::now(),
            waited_dur: Duration::new(0, 0),
        }
    }

    pub fn advance(&mut self) -> bool {
        let now = Instant::now();

        self.waited_dur += self.last_time.elapsed();
        self.last_time = now;

        if self.waited_dur > self.dur {
            self.waited_dur -= self.dur;
            return true;
        }

        false
    }

    pub fn if_elapsed<F>(&mut self, f: F) -> bool
    where
        F: FnOnce(),
    {
        if self.advance() {
            f();
            return true;
        }

        false
    }
}
