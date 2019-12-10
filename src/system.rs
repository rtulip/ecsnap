use crate::{Component, Entity, Resource, World};
use std::fmt::Debug;

/// Trait used to define what kind of data can be used to Query in a `System`.
/// `SystemData` can `fetch` data from an entity if it has the system data. Additionaly,
/// `SystemData` can `set` the data to an entity.
///
/// `SystemData` can be a single `Component` or a tuple of `Components`.
/// #TODO:
///     Allow for generic Component tuple instead of just (A,B).
pub trait SystemData: Sized + Clone + Debug {
    /// Returns the `SystemData` of an `Entity` if the `Entity` has the requisite
    /// `Components`. If the `Entity` doesn't have the requisite `Components` than `None`
    /// is returned.
    fn fetch(e: &Entity) -> Option<Self>;
    /// Defines the behaviour for updateing an `Entities` data to some new `SystemData`.
    fn set(self, e: &mut Entity);
}

impl SystemData for () {
    fn fetch(_e: &Entity) -> Option<Self> {
        None
    }
    fn set(self, _e: &mut Entity) {}
}

impl<C> SystemData for C
where
    C: Component,
{
    fn fetch(e: &Entity) -> Option<Self> {
        if let Some(c) = e.get_component::<C>() {
            Some((*c).clone())
        } else {
            None
        }
    }
    fn set(self, e: &mut Entity) {
        e.add_component::<C>(self);
    }
}

impl<A, B> SystemData for (A, B)
where
    A: Component,
    B: Component,
{
    fn fetch(e: &Entity) -> Option<Self> {
        match (e.get_component::<A>(), e.get_component::<B>()) {
            (Some(a), Some(b)) => Some(((*a).clone(), (*b).clone())),
            _ => None,
        }
    }
    fn set(self, e: &mut Entity) {
        e.add_component::<A>(self.0);
        e.add_component::<B>(self.1);
    }
}

/// Trait defining how to access world `Resources`.
pub trait ResourceData: Sized + Debug + Clone {
    /// Fetches the `Resources for the `World`.
    fn fetch_res(w: &World) -> Option<Self>;
    /// Sets the Resourcedata to the World.
    fn set_res(self, w: &mut World);
}

impl ResourceData for () {
    fn fetch_res(_w: &World) -> Option<Self> {
        None
    }
    fn set_res(self, _w: &mut World) {}
}

impl<R: Resource> ResourceData for R {
    fn fetch_res(w: &World) -> Option<Self> {
        match w.get_resource::<R>() {
            Some(a) => Some((*a).clone()),
            _ => None,
        }
    }
    fn set_res(self, w: &mut World) {
        w.add_resource::<R>(self);
    }
}

/// Trait defining a generic System. Any `Entity` with that doens't return `None` to
/// `System::Data::fetch` will have `run` called on its Data.
pub trait System {
    /// Defines the type of data to be queried.
    type Data: SystemData;
    /// Defines which `Resources` should be queried.
    type Resources: ResourceData;
    /// Defines the behaviour of the system. Gets called in World::system_dispatch.
    fn run(&mut self, _data: &mut Self::Data, _res: &Self::Resources) {}
    /// Deifnes the behaviour for updating the system. Gets called in World::dispatch_resource_update.
    fn update_resource(&self, _res: &mut Self::Resources) {}
}

#[cfg(test)]
mod test_system {

    use crate::{Component, System, World};

    #[test]
    fn ideal() {
        #[derive(Debug, Clone, Component)]
        struct Pos {
            x: f64,
            y: f64,
        }

        #[derive(Debug, Clone, Component)]
        struct Vel {
            x: f64,
            y: f64,
        }

        let mut world = World::default();
        world
            .create_entity()
            .with(Pos { x: 0.0, y: 0.0 })
            .with(Vel { x: 1.6, y: -4.5 })
            .build();

        struct ReadSys {}

        impl System for ReadSys {
            type Data = (Pos, Vel);
            type Resources = ();
            fn run(&mut self, data: &mut Self::Data, _res: &Self::Resources) {
                let (pos, vel) = data;
                println!("Pos: {:?}", pos);
                println!("Vel: {:?}", vel);
                pos.x += 10.0;
                pos.y += 5.0;
            }
        }

        let mut rs = ReadSys {};
        println!("Dispatching System!");
        world.dispatch_system(&mut rs);
        world.dispatch_system(&mut rs);
    }
}
