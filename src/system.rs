use crate::Entity;

pub trait SystemData<'a>: Sized {
    fn fetch(e: &'a mut Entity) -> Option<Self>;
}

pub type Read<'a, T> = &'a T;
pub type Write<'a, T> = &'a mut T;

impl<'a, T> SystemData<'a> for Read<'a, T> 
    where T: 'static {
    fn fetch(e: &'a mut Entity) -> Option<Read<'a, T>> {
        e.get_component::<T>()
    }
}
impl<'a, T> SystemData<'a> for Write<'a, T> 
    where T: 'static {
    fn fetch(e: &'a mut Entity) -> Option<Write<'a, T>> {
        e.get_mut_component::<T>()
    }
}

pub trait System<'a> {
    type Data: SystemData<'a>; 
    fn run(&mut self, data: Self::Data);
}

#[cfg(test)]
mod test_system {

    use crate::{World, System, Read, Write};

    #[test]
    fn ideal() {
        
        #[derive(Debug)]
        struct Pos {
            x: f64,
            y: f64,
        }

        let mut world = World::default();
        world.create_entity().with(Pos { x: 0.0, y: 0.0 }).build();

        struct ReadSys {}

        impl<'a> System<'a> for ReadSys {
            type Data = Read<'a, Pos>;

            fn run(&mut self, data: Self::Data) {
                println!("Pos: {:?}", data);
            }
        }

        struct WriteSys {
            val: f64,
        }

        impl<'a> System<'a> for WriteSys{
            type Data = Write<'a, Pos>;
            fn run(&mut self, data: Self::Data) {
                println!("Pos: {:?}", data);
                println!("MUTATING!");
                data.x += self.val;
                data.y += self.val;
                println!("Pos: {:?}", data);
            }
        }

        let mut rs = ReadSys {};
        let mut ws = WriteSys { val: 1.0 };

        world.dispatch_system(&mut rs);
        world.dispatch_system(&mut ws);
        world.dispatch_system(&mut ws);
    }

}