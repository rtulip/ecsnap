use std::fmt::Debug;
pub trait Component: 'static + Clone + Copy + Debug + Sized {}