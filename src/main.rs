use space_station::{SpaceShip, GenericInfo, MotherShip};
fn main() {
    let mut zeus = SpaceShip::new("Zeus");
    let mut ada = MotherShip::new("Ada");
    zeus.recharge(&mut ada);
}
