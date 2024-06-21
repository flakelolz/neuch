use crate::prelude::*;

#[derive(Default)]
pub struct Context {
    pub next: Option<Box<dyn State>>,
}
