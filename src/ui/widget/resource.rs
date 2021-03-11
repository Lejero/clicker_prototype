use druid::im::{vector, Vector};
use druid::widget::{Container, CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, Padding as Pad, TextBox, WidgetExt};
use druid::*;

use crate::game::resource::Resource;
use crate::ui::events::UPDATE_RESOURCE;
use crate::ui::widget::util;
use crate::ui::TerminateOnCloseDelegate;

use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::vec::*;

pub struct ResourceWidget {
    element: Box<dyn Widget<WidgetData>>,
    min_size: Size,
}

impl ResourceWidget {
    pub fn new() -> Self {
        let me = ResourceWidget {
            element: build_resource_row(false).boxed(),
            min_size: Size::new(100.0, 20.0),
        };

        me
    }
}

// type WidgetData = (String, Resource);
type WidgetData = Resource;

impl Widget<WidgetData> for ResourceWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut WidgetData, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(UPDATE_RESOURCE) && cmd.get_unchecked(UPDATE_RESOURCE).name == data.name => {
                // let cmd_data = cmd.get_unchecked(UPDATE_RESOURCE).clone();
                *data = cmd.get_unchecked(UPDATE_RESOURCE).clone();

                self.element.event(ctx, event, data, env);
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &WidgetData, env: &Env) {
        self.element.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &WidgetData, data: &WidgetData, env: &Env) {
        if old_data != data {
            ctx.request_paint();
            self.element.update(ctx, old_data, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &WidgetData, env: &Env) -> Size {
        let child_bc = BoxConstraints::new(bc.min(), bc.max());
        let child_size = self.element.layout(ctx, &child_bc, data, env);

        BoxConstraints::new(child_size, bc.max()).min()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &WidgetData, env: &Env) {
        let size = ctx.size();
        let rect = size.to_rect();

        self.element.paint(ctx, data, env);
    }
}

fn build_resource_row(_actively_collected: bool) -> impl Widget<WidgetData> {
    let flex = util::default_flex_row()
        .with_child(Label::dynamic(|data: &String, _| format!("{}:", data)).lens(Resource::name))
        .with_child(Label::dynamic(|data: &i64, _| format!("{}", data)).lens(Resource::quantity));
    // .lens(lens!(WidgetData));

    util::default_container(flex)
}
