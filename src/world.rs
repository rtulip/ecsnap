use crate::{
    Component,
    Entity,
};
pub struct World {

}

impl World {

    pub fn register_component_with_storage<C: Component>(&mut self) {}
    
    // pub fn add_component_to_storage<C: Component>(&mut self, component: C) -> usize {}
    
    // pub fn get_component<C: Component>(&self, index: usize) -> &C {}

    // pub fn create_entity(&mut self) -> Entity {}
    
    pub fn add_component_to_entity<C: Component>(&mut self, entity: Entity, component: C) {}
    
    // pub fn get_component_for_entity<C: Component>(&self, entity: Entity) -> Option<&C> {}
    
    // pub fn get_component_index_for_entity<C: Component>(&self, entity: Entity) -> Option<usize> {}

    // pub fn remove_component_from_storage<C: Component>(&mut self, component_index: usize) -> C {}

    pub fn remove_component_from_entity<C: Component>(&mut self, entity: Entity, component: &C) {}

    fn destroy_entity(&mut self, entity: Entity) {}

}