use std::fmt::Debug;
/// Trait requirements for all Components.
pub trait Component: 'static + Clone + Debug + Sized {}
