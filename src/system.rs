use crate::{Component, Entity};

pub trait SystemData<'a>: Sized + Clone {
    fn fetch(e: &'a Entity) -> Option<Self>;
    fn set(self, e:&'a mut Entity);
}

pub type Query<'a, C> = &'a C;

impl<'a, C> SystemData<'a> for Query<'a, C>
    where C: Component {
    fn fetch(e: &'a Entity) -> Option<Self> {
        e.get_component::<C>()
    }
    fn set(self, e: &'a mut Entity) {
        e.add_component::<C>(*self);
    }
}

impl<'a, A, B> SystemData<'a> for (Query<'a, A>, Query<'a, B>) 
    where A: Component, B: Component {
    fn fetch(e: &'a Entity) -> Option<Self> {
        match (e.get_component::<A>(),  e.get_component::<B>()) {
            (Some(a), Some(b)) => Some((a,b)),
            _ => None
        }
    }
    fn set(self, e: &'a mut Entity) {
        e.add_component::<A>(*self.0);
        e.add_component::<B>(*self.1);
    }

}

pub trait System<'a> {
    type Data: SystemData<'a>; 
    fn run(&mut self, data: &mut Self::Data);
}

#[cfg(test)]
mod test_system {

    use crate::{World, System, Component, Query};

    #[test]
    fn ideal() {
        
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

        let mut world = World::default();
        world
            .create_entity()
            .with(Pos { x: 0.0, y: 0.0 })
            .with(Vel { x: 1.6, y: -4.5} )
            .build();

        struct ReadSys {}

        impl<'a> System<'a> for ReadSys {
            type Data = (Query<'a, Pos>, Query<'a, Vel>);

            fn run(&mut self, data: &mut Self::Data) {
                let (pos, vel) = data;
                println!("Pos: {:?}", pos);
                println!("Vel: {:?}", vel);
            }
        }

        let mut rs = ReadSys {};
        println!("Dispatching System!");
        world.dispatch_system(&mut rs);
    }

}