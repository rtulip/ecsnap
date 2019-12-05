use crate::{Component, Entity};

pub trait SystemData<'a>: Sized {
    fn fetch(e: &'a Entity) -> Option<&Self>;
}

impl<'a, C> SystemData<'a> for C
    where C: Component {
    fn fetch(e: &'a Entity) -> Option<&Self> {
        e.get_component::<C>()
    }
}

pub trait System<'a> {
    type Data: SystemData<'a>; 
    fn run(&mut self, data: Self::Data);
}

#[cfg(test)]
mod test_system {

    use crate::{World, System, Component};

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
            type Data = Pos;

            fn run(&mut self, data: Self::Data) {
                println!("Pos: {:?}", data);
            }
        }

        let mut rs = ReadSys {};
        
        world.dispatch_system(&mut rs);
    }

}