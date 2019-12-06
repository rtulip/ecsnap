use crate::{Component, Eid, Entity, EntityBuilder, System, SystemData};
use std::any::TypeId;
use std::collections::{HashMap, HashSet};

/// A container for all the `Entities`.
#[derive(Debug, Default)]
pub struct World {
    component_ids: HashSet<TypeId>,
    entities: HashMap<Eid, Entity>,
    next_entity_id: Eid,
}

impl World {
    /// Registers a component which can be used by a system (TODO).
    ///
    /// # Example
    /// ```
    /// extern crate ecsnap;
    /// use ecsnap::{World, Component};
    ///
    /// #[derive(Debug, Clone, Copy)]
    /// struct Pos {
    ///     x: f64,
    ///     y: f64,
    /// }
    ///
    /// impl Component for Pos {}
    ///
    /// let mut world = World::default();
    /// world.register_component::<Pos>();
    /// ```
    pub fn register_component<C: Component>(&mut self) -> bool {
        self.component_ids.insert(TypeId::of::<C>())
    }

    /// Creates an `EntityBuilder` to start creating an `Entity`. Calling .build() on the
    /// `EntityBuilder` will add the constructed `Entity` to the `World`.
    ///
    /// # Example
    /// ```
    /// extern crate ecsnap;
    /// use ecsnap::{Component, World};
    ///
    /// #[derive(Debug, Clone, Copy)]
    /// struct Pos {
    ///     x: f64,
    ///     y: f64,
    /// }
    /// impl Component for Pos {}
    ///
    /// let mut world = World::default();
    /// world.create_entity()
    ///     .with(Pos { x: 0.0, y: 0.0 })
    ///     .build();
    /// ```
    pub fn create_entity(&mut self) -> EntityBuilder {
        EntityBuilder::new(self)
    }

    pub(crate) fn insert_entity(&mut self, e: Entity) -> Eid {
        let id = self.next_entity_id;
        self.entities.insert(id, e);
        self.next_entity_id += 1;
        id
    }

    #[allow(dead_code)]
    pub(crate) fn get_component_for_entity<C: Component>(&self, entity: &Eid) -> Option<&C> {
        if let Some(e) = self.entities.get(entity) {
            e.get_component::<C>()
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub(crate) fn remove_component_from_entity<C: Component>(
        &mut self,
        entity: &Eid,
    ) -> Option<Box<C>> {
        if let Some(e) = self.entities.get_mut(entity) {
            e.remove_component::<C>()
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub(crate) fn destroy_entity(&mut self, entity: &Eid) -> Option<Entity> {
        self.entities.remove(entity)
    }

    /// Runs a system on the `World`.
    ///
    /// # Example
    /// ```
    /// extern crate ecsnap;
    /// use ecsnap::{Component, System, World};
    ///
    /// #[derive(Debug, Clone, Copy)]
    /// struct Pos {
    ///     x: f64,
    ///     y: f64,
    /// }
    ///
    /// #[derive(Debug, Clone, Copy)]
    /// struct Vel {
    ///     x: f64,
    ///     y: f64,
    /// }
    ///
    /// impl Component for Pos {}
    /// impl Component for Vel {}
    ///
    /// struct MovementSystem {
    ///     dt: f64,         
    /// }
    ///
    /// impl System for MovementSystem {
    ///     type Data = (Pos, Vel);
    ///     fn run(&mut self, data: &mut Self::Data){
    ///         let (pos, vel) = data;
    ///         pos.x += vel.x * self.dt;
    ///         pos.y += vel.y * self.dt;
    ///         println!("Updated Position! {:?}", pos);
    ///         
    ///     }
    /// }
    ///
    /// let mut mvt = MovementSystem { dt : 0.05 };
    ///
    /// let mut world = World::default();
    /// world
    ///     .create_entity()
    ///     .with(Pos {x: 0.0, y: 0.0})
    ///     .with(Vel {x: 10.0, y: 10.0})
    ///     .build();
    ///
    /// world
    ///     .create_entity()
    ///     .with(Pos {x: 0.0, y: 0.0})
    ///     .build();
    ///
    /// world.dispatch_system(&mut mvt);
    /// world.dispatch_system(&mut mvt);
    ///
    /// ```
    pub fn dispatch_system<S: System>(&mut self, sys: &mut S) {
        for entity in self.entities.values_mut() {
            if let Some(data) = S::Data::fetch(entity) {
                let mut new_data = data.clone();
                sys.run(&mut new_data);
                println!("Manipulated Data: {:?}", new_data);
                entity.set::<S>(new_data);
            }
        }
    }
}

#[cfg(test)]
mod test_world {

    use crate::{Component, World};
    #[test]
    fn test_register_component() {
        #[derive(Debug, Clone, Copy)]
        struct Pos {
            _x: f64,
            _y: f64,
        }

        impl Component for Pos {}

        let mut world: World = Default::default();
        let val = world.register_component::<Pos>();

        assert!(val);
    }

    #[test]
    fn test_add_component_to_entity() {
        #[derive(Debug, Clone, Copy)]
        struct Pos {
            x: f64,
            y: f64,
        }

        #[derive(Debug, Clone, Copy)]
        struct Vel {
            x: f64,
            y: f64,
        }

        impl Component for Pos {}
        impl Component for Vel {}

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
        #[derive(Debug, Clone, Copy)]
        struct Pos {
            x: f64,
            y: f64,
        }

        #[derive(Debug, Clone, Copy)]
        struct Vel {
            x: f64,
            y: f64,
        }

        impl Component for Pos {}
        impl Component for Vel {}

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
        #[derive(Debug, Clone, Copy)]
        struct Pos {
            x: f64,
            y: f64,
        }

        #[derive(Debug, Clone, Copy)]
        struct Vel {
            x: f64,
            y: f64,
        }

        impl Component for Pos {}
        impl Component for Vel {}

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
