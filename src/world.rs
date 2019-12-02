use crate::{
    Entity,
    Eid,
};
use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Default)]
pub struct World {
    pub components: HashMap<TypeId, Box<dyn Any>>,
    pub entities: Vec<Entity>,
    next_entity_id: Eid,
}

type Storage<C> = HashMap<Eid, C>;

impl World {

    fn get_component<C: 'static>(&self) -> &Storage<C> {
        self.components.get(&TypeId::of::<C>())
            .unwrap()
            .downcast_ref::<Storage<C>>()
            .unwrap()

    }
    
    fn get_mut_component<C: 'static>(&mut self) -> &mut Storage<C> {
        self.components.get_mut(&TypeId::of::<C>())
            .unwrap()
            .downcast_mut::<Storage<C>>()
            .unwrap()
    }

    pub fn register_component<C: 'static>(&mut self) -> Option<Box<dyn Any>> {
        self.components.insert(TypeId::of::<C>(), Box::new(Storage::<C>::new()))
    }
    
    pub fn create_entity(&mut self) -> Entity {
        let e = Entity {
            id: self.next_entity_id,
        };
        self.entities.push(e);
        self.next_entity_id += 1;
        e
    }
    
    pub fn add_component_to_entity<C: 'static>(&mut self, entity: Entity, component: C) -> Option<C> {
        self.get_mut_component::<C>().insert(entity.id, component)
    }
    
    pub fn get_component_for_entity<C: 'static>(&self, entity: &Entity) -> Option<&C> {
        self.get_component::<C>().get(&entity.id)
    }
    
    pub fn remove_component_from_entity<C: 'static>(&mut self, entity: &Entity) -> Option<C> {
        self.get_mut_component::<C>().remove(&entity.id)
    }

    // fn destroy_entity(&mut self, entity: &Entity) {}

}

#[cfg(test)]
mod test_world {

    use crate::{World};
    #[test]
    fn test_register_component() {

        struct Pos {
            _x: f64, 
            _y: f64,
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

    #[test]
    fn test_remove_component_from_entity() {

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

        let mut world: World = Default::default();
        world.register_component::<Pos>();
        world.register_component::<Vel>();

        let e = world.create_entity();
        
        world.add_component_to_entity(e, Pos {x: 0.0, y: 0.0});
        world.add_component_to_entity(e, Vel {x: 0.0, y: 0.0});

        let e_pos = world.get_component_for_entity::<Pos>(&e);
        let e_vel = world.get_component_for_entity::<Vel>(&e);
        
        assert!(e_pos.is_some());
        assert!(e_pos.unwrap().x == 0.0);
        assert!(e_pos.unwrap().y == 0.0);

        assert!(e_vel.is_some());
        assert!(e_vel.unwrap().x == 0.0);
        assert!(e_vel.unwrap().y == 0.0);
        
        let val = world.remove_component_from_entity::<Vel>(&e);
        assert!(val.is_some());
        let val = val.unwrap();
        assert_eq!(val.x, 0.0);
        assert_eq!(val.y, 0.0);

        let val = world.remove_component_from_entity::<Vel>(&e);
        assert!(val.is_none());
    }


}