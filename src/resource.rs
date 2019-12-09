use std::fmt::Debug;

pub trait Resource: 'static + Clone + Debug + Sized {}
