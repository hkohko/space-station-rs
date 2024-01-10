use space_station::{mother_ship::MotherShip, space_ship::SpaceShip, GenericInfo};
fn main() {
    let mut zeus = SpaceShip::new("Zeus");
    let mut ada = MotherShip::new("Ada");
    ada.display_info();
    zeus.recharge(&mut ada);
    ada.display_info();
}

#[test]
fn current_features() {
    let mut zeus = SpaceShip::new("Zeus");
    let mut ada = MotherShip::new("Ada");
    ada.display_info();
    zeus.recharge(&mut ada);
    ada.display_info();
}
