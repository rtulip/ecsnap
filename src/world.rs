use crate::{
    Component,
    Entity,
    Eid,
    GenericStorage,
};
use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Default)]
pub struct World {
    pub components: HashMap<TypeId, Box<dyn Component>>,
    pub entities: Vec<Entity>,
    next_entity_id: Eid,
}

impl World {

    pub fn register_component<C: Component>(&mut self) -> Option<Box<dyn Any>> {
        self.components.insert(TypeId::of::<C>(), Box::new(C::Storage::new()))
    }
    
    pub fn create_entity(&mut self) -> Entity {
        let e = Entity {
            id: self.next_entity_id,
        };
        self.entities.push(e);
        self.next_entity_id += 1;
        e    
    }
    
    pub fn add_component_to_entity<C: Component>(&mut self, entity: Entity, component: C) {
        let storage = self.components.get_mut(&TypeId::of::<C>()).unwrap().downcast_mut::<C::Storage>().unwrap();
        storage.push(entity.id, component);
    }
    
    // pub fn get_component_for_entity<C: Component>(&self, entity: Entity) -> Option<&C> {}
    
    // pub fn get_component_index_for_entity<C: Component>(&self, entity: Entity) -> Option<usize> {}

    // pub fn remove_component_from_storage<C: Component>(&mut self, component_index: usize) -> C {}

    pub fn remove_component_from_entity<C: Component>(&mut self, entity: Entity, component: &C) {}

    fn destroy_entity(&mut self, entity: Entity) {}

}

#[cfg(test)]
mod test_world {

    use crate::{Component, World, Eid};
    use std::collections::HashMap;
    #[test]
    fn test_register_component() {

        println!("Starting test!");

        struct Pos {
            _x: f64, 
            _y: f64,
        }

        impl Component for Pos {
            type Storage = HashMap<Eid, Self>;
        }

        let mut world: World = Default::default();
        let val = world.register_component::<Pos>();

        assert!(!val.is_some());
        assert!(!world.components.is_empty());
        
    }

}