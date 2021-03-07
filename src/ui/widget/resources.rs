use druid::im::{vector, Vector};
use druid::widget::{CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, Padding as Pad, TextBox, WidgetExt};
use druid::*;

use crate::game_loop::game_loop::{Character, GameState, Resource, Resources};
use crate::ui::ui::{UPDATE_RESOURCE, UPDATE_RESOURCES};
use crate::ui::TerminateOnCloseDelegate;

use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::vec::*;

pub struct ResourcesWidget {
    resources_element: Box<dyn Widget<Resources>>,
}

impl ResourcesWidget {
    pub fn new() -> Self {
        let me = ResourcesWidget {
            resources_element: build_resources_widget().boxed(),
        };

        me
    }
}

impl Widget<Resources> for ResourcesWidget {
    fn event(&mut self, _ctx: &mut EventCtx, event: &Event, data: &mut Resources, _env: &Env) {
        match event {
            // This is where we handle our command.
            Event::Command(cmd) if cmd.is(UPDATE_RESOURCES) => {
                // We don't do much data processing in the `event` method.
                // All we really do is just set the data. This causes a call
                // to `update` which requests a paint. You can also request a paint
                // during the event, but this should be reserved for changes to self.
                // For changes to `Data` always make `update` do the paint requesting.
                //*data = cmd.get_unchecked(UPDATE_RESOURCES).clone();

                //self.resources_element = build_resources_widget().boxed();
                (*data).iron.quantity += 1;
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Resources, env: &Env) {
        self.resources_element.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Resources, data: &Resources, env: &Env) {
        if old_data != data {
            //self.resources_element = Box::new(build_resources_widget());

            ctx.request_paint()
        }
        //self.resources_element.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &Resources, env: &Env) -> Size {
        //let new_bc = bc.max();

        //let child_bc = BoxConstraints::new(bc.min(), bc.max());
        self.resources_element.layout(ctx, bc, data, env);

        //let origin = Point::new(10.0, 10.0);
        //self.resources_element.set_origin(ctx, data, env, origin);

        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Resources, env: &Env) {
        let size = ctx.size();
        let rect = size.to_rect();

        // ctx.fill(rect, &Color::WHITE);
        // let other_rect = Rect::new(rect.x0 + 10.0, rect.y0 + 10.0, (rect.x1 - 10.0) / 2.0, (rect.y1 - 10.0) / 2.0);
        // ctx.fill(other_rect, &Color::RED);

        //println!("{}", rect);
        println!("{}", data.iron);

        self.resources_element.paint(ctx, data, env);
        // ctx.with_child_ctx(rect, |ctx| {
        //     self.resources_element.paint(ctx, data, env);
        // })
        // ctx.with_save(|ctx| {
        //     self.resources_element.paint(ctx, data, env);
        // });
    }
}

pub fn build_resources_widget() -> impl Widget<Resources> {
    Pad::new(
        2.0,
        Flex::column()
            //.with_flex_child(Label::new("Hello World"), 1.0)
            .with_flex_child(build_resource_row(false).lens(Resources::iron), 1.0)
            .with_flex_child(build_resource_row(false).lens(Resources::copper), 1.0)
            .with_flex_child(build_resource_row(false).lens(Resources::tin), 1.0)
            .with_flex_child(build_resource_row(false).lens(Resources::nickel), 1.0)
            .with_flex_child(build_resource_row(false).lens(Resources::coal), 1.0)
            .with_flex_child(build_resource_row(false).lens(Resources::steel), 1.0),
    )
}

fn build_resource_row(_actively_collected: bool) -> impl Widget<Resource> {
    //let name = (format!("{}:", resource_name)).as_str();
    Flex::row()
        .with_flex_child(Label::dynamic(|data: &String, _| format!("{}:", data)).lens(Resource::name), 2.0)
        .with_flex_child(Label::dynamic(|data: &i64, _| format!("{}", data)).lens(Resource::quantity), 1.0)
}
