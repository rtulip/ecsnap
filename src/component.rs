use crate::GenericStorage;
use std::any::Any;

pub trait Component: Any + Sized {
    type Storage: GenericStorage<Self>;
}
