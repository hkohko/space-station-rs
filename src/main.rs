use space_station::experimentals::experimental_main;
use space_station::prelude::*;
fn main() {
    let new_world = World::randomize();
    let mut ada = MotherShip::new("Ada", &new_world);
    let mut zeus = SpaceShip::new("Zeus", &new_world);
    zeus.recharge(&mut ada);
}
