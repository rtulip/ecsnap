use crate::Eid;
use std::collections::HashMap;

pub trait GenericStorage<T> {
    fn new() -> Self
    where
        Self: Sized;
    fn push(&mut self, id: Eid, value: T) -> Option<T>;
    fn get(&self, id: &Eid) -> Option<&T>;
    fn len(&self) -> usize;
    fn remove(&mut self, id: &Eid) -> Option<T>;
}

impl<T> GenericStorage<T> for HashMap<Eid, T>{
    fn new() -> Self {
        HashMap::new()
    }

    fn push(&mut self, id: Eid, value: T) -> Option<T> {
        self.insert(id, value)
    }

    fn get(&self, id: &Eid) -> Option<&T> {
        HashMap::get(self, id)
    }

    fn len(&self) -> usize {
        HashMap::len(self)
    }

    fn remove(&mut self, id: &Eid) -> Option<T> {
        HashMap::remove(self, id)
    }
}