pub struct Action {}

impl Action {
    pub fn execute() {}
}

pub trait Act {
    fn update();
}

// trait EntityBase {
//     fn id() -> String;
//     fn name() -> String;
// }
