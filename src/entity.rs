use std::any::{Any, TypeId};
use std::collections::HashMap;
use crate::Component;

pub type Eid = usize;
#[derive(Default)]
pub struct Entity {
    pub components: HashMap<TypeId, Box<dyn Any>>
}

impl Entity {
    pub fn add_component<C: Component>(&mut self, component: C) -> Option<Box<dyn Any>> {
        self.components.insert(TypeId::of::<C>(), Box::new(component))
    }

    pub fn get_component<C: Component>(&self) -> Option<&C> {
        if let Some(bx) = self.components.get(&TypeId::of::<C>()) {
            bx.downcast_ref::<C>()
        } else {
            None
        }
    }

    pub fn get_mut_component<C: Component>(&mut self) -> Option<&mut C> {
        self.components.get_mut(&TypeId::of::<C>())
            .unwrap()
            .downcast_mut::<C>()
    }

    pub fn remove_component<C: Component>(&mut self) -> Option<Box<dyn Any>> {
        self.components.remove(&TypeId::of::<C>()) 
    }
}