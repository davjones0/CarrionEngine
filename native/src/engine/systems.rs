use specs::{ReadStorage, System, WriteStorage};
use engine::components::{Movement, Position};

pub struct ProcessMovement;

impl<'a> System<'a> for ProcessMovement {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Movement>);

    fn run(&mut self, (mut pos, movement): Self::SystemData) {
        use specs::Join;
        for (pos, movement) in (&mut pos, &movement).join() {
            pos.x += movement.x;
            pos.y += movement.y;
        }
    }
}


pub struct ClearMovement;

impl<'a> System<'a> for ClearMovement {
    type SystemData = WriteStorage<'a, Movement>;

    fn run (&mut self, mut movement: Self::SystemData) {
        use specs::Join;
        for movement in (&mut movement).join() {
            movement.x = 0;
            movement.y = 0;            
        }
    }
}