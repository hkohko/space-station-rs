use space_station::experimentals::experimental_main;
use space_station::prelude::*;
fn main() {
    let new_world = World::randomize(WorldSize::new(100));
    let mut ada = MotherShip::new("Ada", &new_world);
    let mut zeus = SpaceShip::new("Zeus", &new_world);
    zeus.ping();
}
