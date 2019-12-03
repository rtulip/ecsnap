pub trait SystemData {}

pub trait System {
    type Data: SystemData; 
    fn run(&mut self, data: Self::Data);
}

#[cfg(test)]
mod test_system {

    use crate::World;

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

        // impl System for MvtSys {
        //     Data = Read<Pos>;

        //     fn run(&mut self, data: Self::Data) {
        //         println!("Pos: {:?}", data);
        //     }
        // }
    }

}