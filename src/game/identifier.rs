use std::fmt::Display;

trait Identifier<T>
where
    T: Display,
{
    fn id() -> T;
}
