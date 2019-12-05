use crate::{Component, Entity};

pub trait SystemData<'a>: Sized {
    fn fetch(e: &'a Entity) -> Option<Self>;
}

pub type Query<'a, C> = &'a C;

impl<'a, C> SystemData<'a> for Query<'a, C>
    where C: Component {
    fn fetch(e: &'a Entity) -> Option<Self> {
        e.get_component::<C>()
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
}

pub trait System<'a> {
    type Data: SystemData<'a>; 
    fn run(&mut self, data: Self::Data);
}

#[cfg(test)]
mod test_system {

    use crate::{World, System, Component, Query};

    #[test]
    fn ideal() {
        
        #[derive(Debug, Clone)]
        struct Pos {
            x: f64,
            y: f64,
        }
        impl Component for Pos {}

        let mut world = World::default();
        world.create_entity().with(Pos { x: 0.0, y: 0.0 }).build();

        struct ReadSys {}

        impl<'a> System<'a> for ReadSys {
            type Data = Query<'a, Pos>;

            fn run(&mut self, data: Self::Data) {
                println!("Pos: {:?}", data);
            }
        }

        let mut rs = ReadSys {};
        
        world.dispatch_system(&mut rs);
    }

}