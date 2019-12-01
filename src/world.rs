use crate::{
    Component,
    Entity,
    GenericStorage,
};
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct World {
    pub components: HashMap<TypeId, Box<dyn Any>>,
}

impl World {

    pub fn register_component<C: Component>(&mut self) {
        self.components.insert(TypeId::of::<C>(), Box::new(C::Storage::new()));
    }
    
    // pub fn add_component_instance<C: Component>(&mut self, component: C) -> usize {}
    
    // pub fn create_entity(&mut self) -> Entity {}
    
    pub fn add_component_to_entity<C: Component>(&mut self, entity: Entity, component: C) {}
    
    // pub fn get_component_for_entity<C: Component>(&self, entity: Entity) -> Option<&C> {}
    
    // pub fn get_component_index_for_entity<C: Component>(&self, entity: Entity) -> Option<usize> {}

    // pub fn remove_component_from_storage<C: Component>(&mut self, component_index: usize) -> C {}

    pub fn remove_component_from_entity<C: Component>(&mut self, entity: Entity, component: &C) {}

    fn destroy_entity(&mut self, entity: Entity) {}

}