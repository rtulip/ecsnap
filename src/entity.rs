use crate::World;
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub type Eid = usize;
#[derive(Default)]
pub struct Entity {
    pub components: HashMap<TypeId, Box<dyn Any>>,
}

impl Entity {
    pub fn add_component<C: 'static>(&mut self, component: C) -> Option<Box<C>> {
        if let Some(bx) = self
            .components
            .insert(TypeId::of::<C>(), Box::new(component))
        {
            if let Ok(comp) = bx.downcast::<C>() {
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
        self.components
            .get_mut(&TypeId::of::<C>())
            .unwrap()
            .downcast_mut::<C>()
    }

    pub fn remove_component<C: 'static>(&mut self) -> Option<Box<C>> {
        if let Some(bx) = self.components.remove(&TypeId::of::<C>()) {
            if let Ok(comp) = bx.downcast::<C>() {
                Some(comp)
            } else {
                panic!();
            }
        } else {
            None
        }
    }
}

pub struct EntityBuilder<'a> {
    entity: Entity,
    world: &'a mut World,
}

impl<'a> EntityBuilder<'a> {
    pub fn new(world: &'a mut World) -> Self {
        EntityBuilder {
            entity: Entity::default(),
            world,
        }
    }

    pub fn with<C: 'static>(mut self, component: C) -> Self {
        self.entity.add_component(component);
        self
    }

    pub fn build(self) -> Eid {
        self.world.insert_entity(self.entity)
    }
}

#[cfg(test)]
mod entity_tests {

    use crate::World;

    #[test]
    fn test_entity_bulider() {
        struct Pos {
            x: f64,
            y: f64,
        }

        let mut world = World::default();
        let e = world.create_entity().with(Pos { x: 0.0, y: 0.0 }).build();

        let pos = world.get_component_for_entity::<Pos>(&e);
        assert!(pos.is_some());
        let pos = pos.unwrap();
        assert_eq!(pos.x, 0.0);
        assert_eq!(pos.y, 0.0);
    }
}
