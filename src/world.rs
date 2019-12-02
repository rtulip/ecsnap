use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};
use crate::{Component, Entity, Eid};

pub struct World {
    component_ids: HashSet<TypeId>,
    entities: HashMap<Eid, Entity>,
    next_entity_id: Eid
}

impl World {

    pub fn register_component<C: Component>(&mut self) -> bool {
        self.component_ids.insert(TypeId::of::<C>())
    }
    
    pub fn create_entity(&mut self) -> Eid {
        let id = self.next_entity_id;
        self.entities.insert(id, Entity::default());
        self.next_entity_id += 1;
        id
    }
    
    pub fn add_component_to_entity<C: Component>(&mut self, entity: &Eid, component: C) -> Option<Box<dyn Any>>{
        self.entities.get_mut(entity)
            .unwrap()
            .add_component(component)
    }
    
    pub fn get_component_for_entity<C: Component>(&self, entity: &Eid) -> Option<&C> {
        self.entities.get(entity)
            .unwrap()
            .get_component::<C>()
    }
    
    pub fn remove_component_from_entity<C: Component>(&mut self, entity: &Eid) -> Option<Box<dyn Any>> {
        self.entities.get_mut(entity)
            .unwrap()
            .remove_component::<C>() 
            
    }

    // fn destroy_entity(&mut self, entity: &Entity) {}
}