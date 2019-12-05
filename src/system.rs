use crate::{Component, Entity};
use std::fmt::Debug;

pub trait SystemData: Sized + Clone + Copy + Debug {
    fn fetch(e: &Entity) -> Option<Self>;
    fn set(self, e: &mut Entity);
}

impl<C> SystemData for C
    where C: Component {
    fn fetch(e: &Entity) -> Option<Self> {
        if let Some(c) = e.get_component::<C>() {
            Some(*c)
        } else {
            None
        }
    }
    fn set(self, e: &mut Entity) {
        e.add_component::<C>(self);
    }
}

impl<A, B> SystemData for (A, B) 
    where A: Component, B: Component {
    fn fetch(e: &Entity) -> Option<Self> {
        match (e.get_component::<A>(),  e.get_component::<B>()) {
            (Some(a), Some(b)) => Some((*a,*b)),
            _ => None
        }
    }
    fn set(self, e: &mut Entity) {
        e.add_component::<A>(self.0);
        e.add_component::<B>(self.1);
    }

}

pub trait System {
    type Data: SystemData; 
    fn run(&mut self, data: &mut Self::Data);
}

#[cfg(test)]
mod test_system {

    use crate::{World, System, Component};

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

        impl System for ReadSys {
            type Data = (Pos,Vel);

            fn run(&mut self, data: &mut Self::Data) {
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