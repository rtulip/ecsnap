use crate::{Component, System, SystemData, World};
use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Type for entity identifier
pub type Eid = usize;

/// A collection for a series of components.
#[derive(Debug, Default)]
pub struct Entity {
    /// A Hashmap used to store the components of the entities.
    pub components: HashMap<TypeId, Box<dyn Any>>,
}

impl Entity {
    /// Adds a component C to the `Entity`. If the `Entity` didn't have a component of
    /// type C already, None is returned. If the `Entity` already had a component of type
    /// C, the component is replaced and the old Boxed component is retured.
    ///
    /// # Example
    /// ```
    /// extern crate ecsnap;
    /// use ecsnap::{Component, Entity};
    ///
    /// #[derive(Debug, Clone, Copy)]
    /// struct Pos {
    ///     x: f64,
    ///     y: f64,
    /// }
    /// impl Component for Pos {}
    ///
    /// let mut e = Entity::default();
    /// e.add_component(Pos { x: 0.0, y: 0.0 });
    ///
    /// let pos = e.get_component::<Pos>().unwrap();
    /// assert_eq!(pos.x, 0.0);
    /// assert_eq!(pos.y, 0.0);
    /// ```
    pub fn add_component<C: Component>(&mut self, component: C) -> Option<Box<C>> {
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

    /// Gets a reference to a component C for an `Entity` if the `Entity` has such a
    /// component, otherwise None is returned.
    ///
    /// # Example
    /// ```
    /// extern crate ecsnap;
    /// use ecsnap::{Component, Entity};
    ///
    /// #[derive(Debug, Clone, Copy)]
    /// struct Pos {
    ///     x: f64,
    ///     y: f64,
    /// }
    /// impl Component for Pos {}
    ///
    /// let mut e = Entity::default();
    /// e.add_component(Pos { x: 0.0, y: 0.0 });
    ///
    /// let pos = e.get_component::<Pos>().unwrap();
    /// assert_eq!(pos.x, 0.0);
    /// assert_eq!(pos.y, 0.0);
    /// ```
    pub fn get_component<C: Component>(&self) -> Option<&C> {
        if let Some(bx) = self.components.get(&TypeId::of::<C>()) {
            bx.downcast_ref::<C>()
        } else {
            None
        }
    }

    /// Gets a component C for an `Entity`. If the `Entity` has a Component C, a
    /// reference to the component is returned, otherwise None is returned.
    ///
    /// # Example
    /// ```
    /// extern crate ecsnap;
    /// use ecsnap::{Component, Entity};
    ///
    /// #[derive(Debug, Clone, Copy)]
    /// struct Pos {
    ///     x: f64,
    ///     y: f64,
    /// }
    /// impl Component for Pos {}
    ///
    /// let mut e = Entity::default();
    /// e.add_component(Pos { x: 0.0, y: 0.0 });
    ///
    /// let pos = e.get_component::<Pos>().unwrap();
    /// assert_eq!(pos.x, 0.0);
    /// assert_eq!(pos.y, 0.0);
    /// ```
    pub fn get_mut_component<C: Component>(&mut self) -> Option<&mut C> {
        self.components
            .get_mut(&TypeId::of::<C>())
            .unwrap()
            .downcast_mut::<C>()
    }

    /// Removes a component C from an `Entity` if it had such a component. If it had a
    /// component C, then a Boxed<C> is returned, otherwise, None is returned.
    ///
    /// # Example
    /// ```
    /// extern crate ecsnap;
    /// use ecsnap::{Component, Entity};
    ///
    /// #[derive(Debug, Clone, Copy)]
    /// struct Pos {
    ///     x: f64,
    ///     y: f64,
    /// }
    /// impl Component for Pos {}
    /// 
    /// let mut e = Entity::default();
    /// e.add_component(Pos { x: 0.0, y: 0.0 });
    /// let pos = e.remove_component::<Pos>().unwrap();
    /// assert_eq!((*pos).x, 0.0);
    /// assert_eq!((*pos).y, 0.0);
    /// ```
    pub fn remove_component<C: Component>(&mut self) -> Option<Box<C>> {
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

    /// Sets the `SystemData` of this `Entity`. Is called internally in 
    /// `World::dispatch_system`.
    pub fn set<S: System>(&mut self, data: S::Data) {
        data.set(self);
    }
}

/// A helper struct to construct `Entities` with components.
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
/// impl Component for Pos {}
///
/// let mut world = World::default();
/// world
///     .create_entity()
///     .with(Pos { x: 0.0, y: 0.0})
///     .build();
/// ```
#[derive(Debug)]
pub struct EntityBuilder<'a> {
    entity: Entity,
    world: &'a mut World,
}

impl<'a> EntityBuilder<'a> {
    pub(crate) fn new(world: &'a mut World) -> Self {
        EntityBuilder {
            entity: Entity::default(),
            world,
        }
    }

    /// Adds a component to the `Entity` being constructed.
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
    /// impl Component for Pos {}
    ///
    /// let mut world = World::default();
    /// world
    ///     .create_entity()
    ///     .with(Pos { x: 0.0, y: 0.0})
    ///     .build();
    /// ```
    pub fn with<C: Component>(mut self, component: C) -> Self {
        self.entity.add_component(component);
        self
    }

    /// Finishes building the `Entity` and returns the Eid of the newly bulit entity.
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
    /// impl Component for Pos {}
    ///
    /// let mut world = World::default();
    /// world
    ///     .create_entity()
    ///     .with(Pos { x: 0.0, y: 0.0})
    ///     .build();
    /// ```
    pub fn build(self) -> Eid {
        self.world.insert_entity(self.entity)
    }
}
