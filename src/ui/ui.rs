use druid::im::{vector, Vector};
use druid::widget::{Button, CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, Padding as Pad, TextBox};
use druid::*;

// use crate::game::resource::{Resource, Resources};
use crate::game::{GameMessage, GameState};
use crate::ui::widget::character::CharacterWidget;
use crate::ui::widget::ctrl_panel::PanelWidget;
use crate::ui::widget::resources::ResourcesWidget;
use crate::ui::TerminateOnCloseDelegate;

use std::fmt::Debug;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::vec::*;

pub const MIN_RESOLUTION: Size = Size::new(480.0, 480.0);
pub const DEFAULT_RESOLUTION: Size = Size::new(1200.0, 900.0);

pub fn build_ui() -> impl Widget<GameState> {
    let mut main_element = Flex::row()
        .with_child(ResourcesWidget::new().lens(GameState::resources))
        .with_child(CharacterWidget::new().lens(GameState::main_character))
        .with_child(PanelWidget::new());

    main_element.set_must_fill_main_axis(false);
    main_element.set_cross_axis_alignment(CrossAxisAlignment::Start);
    main_element.set_main_axis_alignment(MainAxisAlignment::Start);

    Pad::new(5.0, main_element)
    //Pad::new(2.0, build_resources_widget().lens(GameState::resources))
}
