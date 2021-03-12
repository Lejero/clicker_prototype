use druid::im::{vector, Vector};
use druid::widget::{
    Container, CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, Padding as Pad, TextBox, WidgetExt,
};
use druid::*;

use crate::game::{resource::Resource, resources::Resources};
use crate::ui::events::{UPDATE_RESOURCE, UPDATE_RESOURCES};
use crate::ui::widget::resource::ResourceWidget;
use crate::ui::widget::util;
use crate::ui::TerminateOnCloseDelegate;

use std::cmp;
use std::cmp::Ord;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::vec::*;

pub struct ResourcesWidget {
    resources_element: Container<Resources>,
    min_size: Size,
}

impl ResourcesWidget {
    pub fn new() -> Self {
        // let mut a = ;
        // .with_child(ResourceWidget::new().lens(Resources::iron).boxed())
        // .with_child(ResourceWidget::new().lens(Resources::copper).boxed())
        // .with_child(ResourceWidget::new().lens(Resources::tin).boxed())
        // .with_child(ResourceWidget::new().lens(Resources::nickel).boxed())
        // .with_child(ResourceWidget::new().lens(Resources::coal).boxed())
        // .with_child(ResourceWidget::new().lens(Resources::steel).boxed());

        ResourcesWidget {
            resources_element: build_resources_widget(), //util::default_container(flex), //build_resources_widget().boxed(),
            min_size: Size::new(100.0, 300.0),
        }
    }

    pub fn empty() -> Self {
        let me = ResourcesWidget {
            resources_element: Container::new(Flex::column()), //build_resources_widget().boxed(),
            min_size: Size::new(100.0, 300.0),
        };

        me
    }
}

impl Widget<Resources> for ResourcesWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Resources, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(UPDATE_RESOURCES) => {
                *data = cmd.get_unchecked(UPDATE_RESOURCES).clone();
            }
            Event::Command(cmd) if cmd.is(UPDATE_RESOURCE) => {}
            _ => (),
        }

        self.resources_element.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Resources, env: &Env) {
        self.resources_element.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Resources, data: &Resources, env: &Env) {
        if old_data != data {
            ctx.request_paint();
            self.resources_element.update(ctx, old_data, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &Resources, env: &Env) -> Size {
        let child_bc = BoxConstraints::new(self.min_size, bc.max());
        let child_size = self.resources_element.layout(ctx, &child_bc, data, env);

        // println!("{}", child_size);
        // BoxConstraints::new(child_size, bc.max()).min()
        child_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Resources, env: &Env) {
        let size = ctx.size();
        let _rect = size.to_rect();

        //ctx.fill(rect, &Color::WHITE);
        // let other_rect = Rect::new(rect.x0 + 10.0, rect.y0 + 10.0, (rect.x1 - 10.0) / 2.0, (rect.y1 - 10.0) / 2.0);
        // ctx.fill(other_rect, &Color::RED);

        //println!("{}", rect);
        // println!("{}", data.iron);

        self.resources_element.paint(ctx, data, env);
    }

    // fn id(&self) -> Option<WidgetId> {
    //     self.resources_element.id()
    // }
}

fn build_resources_widget() -> Container<Resources> {
    let ls = List::new(|| ResourceWidget::new()).lens(Resources::ls);
    util::default_container(ls)
}
