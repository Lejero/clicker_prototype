use druid::im::{vector, Vector};
use druid::widget::{Container, CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, Padding as Pad, TextBox, WidgetExt};
use druid::*;

pub fn default_flex_col<T: Data>() -> Flex<T> {
    let mut flex = Flex::column();

    flex.set_must_fill_main_axis(false);
    flex = flex.cross_axis_alignment(CrossAxisAlignment::Start);
    flex = flex.main_axis_alignment(MainAxisAlignment::Start);

    flex
}
pub fn default_flex_row<T: Data>() -> Flex<T> {
    let mut flex = Flex::row();

    flex.set_must_fill_main_axis(false);
    flex = flex.cross_axis_alignment(CrossAxisAlignment::Start);
    flex = flex.main_axis_alignment(MainAxisAlignment::Start);

    flex
}

pub fn default_container<T: Data>(inner: impl Widget<T> + 'static) -> Container<T> {
    let mut cont = Container::new(inner);
    cont.set_border(Color::rgb8(240, 178, 34), 1.0);
    cont.set_rounded(4.00);

    cont
}
// pub fn default_list<T: Data>(inner: impl Widget<T> + 'static) -> List<T> {
//     let mut ls = List::new(|| inner);

//     ls
// }
