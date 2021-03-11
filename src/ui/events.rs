use druid::Selector;

use crate::game::resource::{Resource, Resources};

pub const UPDATE_RESOURCE: Selector<Resource> = Selector::new("clicker_prototype.update_resource");
pub const UPDATE_RESOURCES: Selector<Resources> = Selector::new("clicker_prototype.update_resources");
