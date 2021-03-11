use druid::Size;

mod simple_delegates;
pub use simple_delegates::TerminateOnCloseDelegate;

pub mod events;
pub mod ui;
pub mod widget;

pub use ui::DEFAULT_RESOLUTION;
pub use ui::MIN_RESOLUTION;
