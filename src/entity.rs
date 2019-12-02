use std::any::{Any, TypeId};
use std::collections::HashMap;

pub type Eid = usize;
#[derive(Default)]
pub struct Entity {
    pub components: HashMap<TypeId, Box<dyn Any>>
}

impl Entity {
    pub fn add_component<C: 'static>(&mut self, component: C) -> Option<Box<C>> {
        if let Some(bx) = self.components.insert(TypeId::of::<C>(), Box::new(component)){
            if let Ok(comp) = bx.downcast::<C>(){
                Some(comp)
            } else {
                panic!();
            }
        } else {
            None
        }

    }

    pub fn get_component<C: 'static>(&self) -> Option<&C> {
        if let Some(bx) = self.components.get(&TypeId::of::<C>()) {
            bx.downcast_ref::<C>()
        } else {
            None
        }
    }

    pub fn get_mut_component<C: 'static>(&mut self) -> Option<&mut C> {
        self.components.get_mut(&TypeId::of::<C>())
            .unwrap()
            .downcast_mut::<C>()
    }

    pub fn remove_component<C: 'static>(&mut self) -> Option<Box<C>> {
        if let Some(bx) = self.components.remove(&TypeId::of::<C>()) {
            if let Ok(comp) = bx.downcast::<C>(){
                Some(comp)
            } else {
                panic!();
            }
        } else {
            None
        }
    }
}