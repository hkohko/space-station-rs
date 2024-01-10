use space_station::{mother_ship::MotherShip, space_ship::SpaceShip, GenericInfo, Move, Coordinates};
fn main() {}


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
    let new_location = Coordinates::new(999,-999);
    zeus.display_info();
    zeus.display_resources();
    zeus.to_location(&new_location);
    zeus.display_info();
    zeus.display_resources();
}
