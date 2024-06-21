use crate::prelude::*;

#[derive(Default)]
pub struct Context {
    pub next: Option<Box<dyn State>>,
    pub elapsed: i32,
    pub duration: i32,
}
