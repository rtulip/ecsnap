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
    pub components: HashMap<TypeId, Box<dyn Any>>,
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
    
    pub fn add_component_to_entity<C: Component>(&mut self, entity: Entity, component: C) -> Option<C> {
        let storage = self.components.get_mut(&TypeId::of::<C>()).unwrap().downcast_mut::<C::Storage>().unwrap();
        storage.push(entity.id, component)
    }
    
    pub fn get_component_for_entity<C: Component>(&self, entity: &Entity) -> Option<&C> {

        let storage = self.components[&TypeId::of::<C>()]
            .downcast_ref::<C::Storage>()
            .unwrap();
        storage.get(&entity.id)
    }
    
    pub fn remove_component_from_entity<C: Component>(&mut self, entity: Entity, component: &C) {}

    fn destroy_entity(&mut self, entity: Entity) {}

}

#[cfg(test)]
mod test_world {

    use crate::{Component, World, Eid};
    use std::collections::HashMap;
    #[test]
    fn test_register_component() {

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

    #[test]
    fn test_add_component_to_entity() {

        #[derive(Debug)]
        struct Pos {
            x: f64, 
            y: f64,
        }

        #[derive(Debug)]
        struct Vel {
            x: f64, 
            y: f64,
        }

        impl Component for Pos {
            type Storage = HashMap<Eid, Self>;
        }

        impl Component for Vel {
            type Storage = HashMap<Eid, Self>;
        }

        let mut world: World = Default::default();
        world.register_component::<Pos>();
        world.register_component::<Vel>();

        let e1 = world.create_entity();
        let e2 = world.create_entity();

        let val = world.add_component_to_entity(e1, Pos {x: 0.0, y: 0.0});
        assert!(val.is_none());
        world.add_component_to_entity(e1, Vel {x: 0.0, y: 0.0});

        world.add_component_to_entity(e2, Pos {x: 3.0, y: 3.0});

        let e1_pos = world.get_component_for_entity::<Pos>(&e1);
        let e1_vel = world.get_component_for_entity::<Vel>(&e1);
        let e2_pos = world.get_component_for_entity::<Pos>(&e2);
        let e2_vel = world.get_component_for_entity::<Vel>(&e2);

        assert!(e1_pos.is_some());
        assert!(e1_pos.unwrap().x == 0.0);
        assert!(e1_pos.unwrap().y == 0.0);

        assert!(e1_vel.is_some());
        assert!(e1_vel.unwrap().x == 0.0);
        assert!(e1_vel.unwrap().y == 0.0);
        
        assert!(e2_pos.is_some());
        assert!(e2_pos.unwrap().x == 3.0);
        assert!(e2_pos.unwrap().y == 3.0);

        assert!(e2_vel.is_none());

    }

}