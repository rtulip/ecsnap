use std::fmt::Debug;
/// Trait requirements for all Resources.
pub trait Resource: 'static + Clone + Sized {}
