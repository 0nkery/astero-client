use specs::{
    System,
};


pub struct Network;

impl<'a> System<'a> for Network {
    type SystemData = ();

    fn run(&mut self, (): Self::SystemData) {

    }
}