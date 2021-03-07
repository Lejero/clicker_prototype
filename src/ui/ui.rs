#![allow(dead_code)]
#![allow(unused_imports)]

use druid::im::{vector, Vector};
use druid::widget::{CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, Padding as Pad, TextBox};
use druid::*;

use crate::game_loop::game_loop::{Character, GameState, Resource, Resources};
use crate::ui::widget::resources::*;
use crate::ui::TerminateOnCloseDelegate;

use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::vec::*;

pub const UPDATE_RESOURCE: Selector<Resource> = Selector::new("clicker_prototype.update_resource");
pub const UPDATE_RESOURCES: Selector<Resources> = Selector::new("clicker_prototype.update_resources");

// pub fn start(state: GameState) -> Result<(), PlatformError> {
// }

pub fn build_ui() -> impl Widget<GameState> {
    // build_resource_widget().lens(GameState::resources)

    Pad::new(2.0, ResourcesWidget::new().lens(GameState::resources))
    //Pad::new(2.0, build_resources_widget().lens(GameState::resources))
}

// fn build_resource_widget() -> impl Widget<Resources> {
//     Pad::new(
//         2.0,
//         Flex::column()
//             .with_flex_child(build_resource_row(false).lens(Resources::iron), 1.0)
//             .with_flex_child(build_resource_row(false).lens(Resources::copper), 1.0)
//             .with_flex_child(build_resource_row(false).lens(Resources::tin), 1.0)
//             .with_flex_child(build_resource_row(false).lens(Resources::nickel), 1.0)
//             .with_flex_child(build_resource_row(false).lens(Resources::coal), 1.0)
//             .with_flex_child(build_resource_row(false).lens(Resources::steel), 1.0),
//     )
// }

fn build_character_widget() -> impl Widget<Character> {
    Pad::new(2.0, Flex::column())
}

fn build_resource_row(_actively_collected: bool) -> impl Widget<Resource> {
    //let name = (format!("{}:", resource_name)).as_str();
    Flex::row()
        .with_flex_child(Label::dynamic(|data: &String, _| format!("{}:", data)).lens(Resource::name), 2.0)
        .with_flex_child(Label::dynamic(|data: &i64, _| format!("{}", data)).lens(Resource::quantity), 1.0)
}
