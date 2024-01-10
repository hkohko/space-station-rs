use space_station::{mother_ship::MotherShip, space_ship::SpaceShip, GenericInfo, Move, Location};
fn main() {}

#[test]
fn recharge_features() {
    let mut zeus = SpaceShip::new("Zeus");
    let mut ada = MotherShip::new("Ada");
    ada.display_resources();
    zeus.recharge(&mut ada);
    ada.display_resources();
}
#[test]
fn move_features() {
    let mut zeus = SpaceShip::new("Zeus");
    let new_location = Location::new(1009,-1002);
    zeus.display_info();
    zeus.to_location(&new_location);
    zeus.display_info();
}
