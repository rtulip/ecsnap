pub trait SystemData<'a> {}

pub type Read<'a, T> = &'a T;
pub type Write<'a, T> = &'a mut T;

impl<'a, T> SystemData<'a> for Read<'a, T> {}
impl<'a, T> SystemData<'a> for Write<'a, T> {}

pub trait System<'a> {
    type Data: SystemData<'a>; 
    fn run(&mut self, data: Self::Data);
}

#[cfg(test)]
mod test_system {

    use crate::{World, System, Read};

    #[test]
    fn ideal() {
        
        #[derive(Debug)]
        struct Pos {
            x: f64,
            y: f64,
        }

        let mut world = World::default();
        world.create_entity().with(Pos { x: 0.0, y: 0.0 }).build();

        struct MvtSys {}

        impl<'a> System<'a> for MvtSys {
            type Data = Read<'a, Pos>;

            fn run(&mut self, data: Self::Data) {
                println!("Pos: {:?}", data);
            }
        }
    }

}