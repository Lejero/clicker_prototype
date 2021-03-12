use druid::im::{vector, Vector};
use druid::widget::{CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, Padding as Pad, TextBox, WidgetExt};
use druid::*;

use crate::game::character::Character;
use crate::ui::events::{UPDATE_RESOURCE, UPDATE_RESOURCES};
use crate::ui::widget::util;
use crate::ui::TerminateOnCloseDelegate;

use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::vec::*;

pub struct CharacterWidget {
    resources_element: Box<dyn Widget<Character>>,
    min_size: Size,
}

impl CharacterWidget {
    pub fn new() -> Self {
        let me = CharacterWidget {
            resources_element: build_character_widget().boxed(),
            min_size: Size::new(200.0, 100.0),
        };

        me
    }
}

impl Widget<Character> for CharacterWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Character, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(UPDATE_RESOURCES) => {}
            _ => (),
        }
        self.resources_element.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Character, env: &Env) {
        self.resources_element.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Character, data: &Character, env: &Env) {
        if old_data != data {
            ctx.request_paint();
            self.resources_element.update(ctx, old_data, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &Character, env: &Env) -> Size {
        let child_bc = BoxConstraints::new(self.min_size, bc.max());
        let child_size = self.resources_element.layout(ctx, &child_bc, data, env);

        child_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Character, env: &Env) {
        let size = ctx.size();
        let rect = size.to_rect();

        self.resources_element.paint(ctx, data, env);
    }

    fn id(&self) -> Option<WidgetId> {
        self.resources_element.id()
    }
}

fn build_character_widget() -> impl Widget<Character> {
    let flex = util::default_flex_col()
        .with_child(Label::dynamic(|data: &String, _| format!("{}", data)).lens(Character::name))
        .with_child(Label::dynamic(|data: &f64, _| format!("{}", data)).lens(Character::health))
        .with_child(Label::dynamic(|data: &f64, _| format!("{}", data)).lens(Character::mana))
        .with_child(Label::dynamic(|data: &f64, _| format!("Collecting: {}", "Nothing")).lens(Character::mana));
    util::default_container(flex)
}
