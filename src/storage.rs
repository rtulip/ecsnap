use std::collections::HashMap;
use crate::Eid;

pub trait GenericStorage<T> {
    fn new() -> Self
    where
        Self: Sized;
    fn push(&mut self, key: Eid, value: T) -> Option<T>;
    fn get(&self, index: &Eid) -> Option<&T>;
    fn len(&self) -> usize;
    fn remove(&mut self, index: &Eid) -> Option<T>;
}
pub type MapStorage<T> = HashMap<Eid, T>;

impl<T> GenericStorage<T> for MapStorage<T> {
    fn new() -> Self {
        return MapStorage::new();
    }

    fn push(&mut self, key: Eid, value: T) -> Option<T> {
        MapStorage::insert(self, key, value)
    }

    fn get(&self, index: &Eid) -> Option<&T> {
        MapStorage::get(self, index)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn remove(&mut self, index: &Eid) -> Option<T> {
        MapStorage::remove(self, index)
    }
    
}