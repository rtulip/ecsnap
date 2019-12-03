use crate::{Eid, Entity, EntityBuilder};
use std::any::TypeId;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct World {
    component_ids: HashSet<TypeId>,
    entities: HashMap<Eid, Entity>,
    next_entity_id: Eid,
}

impl World {
    pub fn register_component<C: 'static>(&mut self) -> bool {
        self.component_ids.insert(TypeId::of::<C>())
    }

    pub fn create_entity<'a>(&mut self) -> EntityBuilder {
        EntityBuilder::new(self)
    }

    pub(crate) fn insert_entity(&mut self, e: Entity) -> Eid {
        let id = self.next_entity_id;
        self.entities.insert(id, e);
        self.next_entity_id += 1;
        id
    }

    pub fn add_component_to_entity<C: 'static>(
        &mut self,
        entity: &Eid,
        component: C,
    ) -> Option<Box<C>> {
        self.entities
            .get_mut(entity)
            .unwrap()
            .add_component(component)
    }

    pub fn get_component_for_entity<C: 'static>(&self, entity: &Eid) -> Option<&C> {
        self.entities.get(entity).unwrap().get_component::<C>()
    }

    pub fn remove_component_from_entity<C: 'static>(&mut self, entity: &Eid) -> Option<Box<C>> {
        self.entities
            .get_mut(entity)
            .unwrap()
            .remove_component::<C>()
    }

    pub fn destroy_entity(&mut self, entity: &Eid) -> Option<Entity> {
        self.entities.remove(entity)
    }
}

#[cfg(test)]
mod test_world {

    use crate::World;
    #[test]
    fn test_register_component() {
        struct Pos {
            _x: f64,
            _y: f64,
        }

        let mut world: World = Default::default();
        let val = world.register_component::<Pos>();

        assert!(val);
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

        let e1 = world
            .create_entity()
            .with(Pos { x: 0.0, y: 0.0 })
            .with(Vel { x: 0.0, y: 0.0 })
            .build();
        let e2 = world.create_entity().with(Pos { x: 3.0, y: 3.0 }).build();

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

        let e = world
            .create_entity()
            .with(Pos { x: 0.0, y: 0.0 })
            .with(Vel { x: 0.0, y: 0.0 })
            .build();

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
        assert_eq!((*val).x, 0.0);
        assert_eq!((*val).y, 0.0);

        let val = world.remove_component_from_entity::<Vel>(&e);
        assert!(val.is_none());
    }

    #[test]
    fn test_destroy_entity() {
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

        let e1 = world
            .create_entity()
            .with(Pos { x: 0.0, y: 0.0 })
            .with(Vel { x: 0.0, y: 0.0 })
            .build();
        let e2 = world.create_entity().with(Pos { x: 0.0, y: 0.0 }).build();

        world.destroy_entity(&e1);

        let dead_e = world.entities.get(&e1);
        assert!(dead_e.is_none());
        let alive_e = world.entities.get(&e2);
        assert!(alive_e.is_some());
    }
}
