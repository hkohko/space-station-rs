use space_station::{GenericInfo, MotherShip, SpaceShip};
fn main() {
    let mut zeus = SpaceShip::new("Zeus");
    let mut ada = MotherShip::new("Ada");
    ada.display_info();
    zeus.recharge(&mut ada);
    ada.display_info();
}
