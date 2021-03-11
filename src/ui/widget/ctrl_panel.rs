use druid::im::{vector, Vector};
use druid::widget::{Button, CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, Padding as Pad, TextBox, WidgetExt};
use druid::*;

use crate::game::character::Character;
use crate::game::{GameMessage, GameState};
use crate::ui::events::{UPDATE_RESOURCE, UPDATE_RESOURCES};
use crate::ui::widget::util;

use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::vec::*;

pub struct PanelWidget {
    resources_element: Box<dyn Widget<GameState>>,
    min_size: Size,
}

impl PanelWidget {
    pub fn new() -> Self {
        let me = PanelWidget {
            resources_element: build_panel_widget().boxed(),
            min_size: Size::new(200.0, 100.0),
        };

        me
    }
}

impl Widget<GameState> for PanelWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut GameState, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(UPDATE_RESOURCES) => {}
            _ => (),
        }
        self.resources_element.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &GameState, env: &Env) {
        self.resources_element.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &GameState, data: &GameState, env: &Env) {
        if old_data != data {
            ctx.request_paint();
            self.resources_element.update(ctx, old_data, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &GameState, env: &Env) -> Size {
        let child_bc = BoxConstraints::new(self.min_size, bc.max());
        let child_size = self.resources_element.layout(ctx, &child_bc, data, env);

        child_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &GameState, env: &Env) {
        let size = ctx.size();
        let rect = size.to_rect();

        self.resources_element.paint(ctx, data, env);
    }

    fn id(&self) -> Option<WidgetId> {
        self.resources_element.id()
    }
}

fn build_panel_widget() -> impl Widget<GameState> {
    // let flex = util::default_flex_col()
    //     .with_child(Button::new("Mine Coal").on_click(|_, data: &mut GameState, _| {
    //         data.msg_tx.send(GameMessage::MINE_RESOURCE("Coal")).expect("Message to game loop from ui failed to send");
    //     }))
    //     .with_child(Button::new("Mine Nickel").on_click(|_, data: &mut GameState, _| {
    //         data.msg_tx.send(GameMessage::MINE_RESOURCE("Nickel")).expect("Message to game loop from ui failed to send");
    //     }));

    let flex = util::default_flex_col()
        .with_child(Button::new("Mine Coal").on_click(|_, data: &mut GameState, _| {
            GameMessage::msg_mine_resource(data.msg_tx.clone(), "Coal");
        }))
        .with_child(Button::new("Mine Nickel").on_click(|_, data: &mut GameState, _| {
            GameMessage::msg_mine_resource(data.msg_tx.clone(), "Nickel");
        }));
    util::default_container(flex)
}
