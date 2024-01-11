use space_station::prelude::*;
fn main() {}
#[test]
fn randomize_stuff() {
    let new_coord = Coordinates::randomize();
    dbg!(&new_coord);
    let new_resources = Resources::randomize(50);
    dbg!(&new_resources);
    let new_env_resource = EnvResource::new(50);
    dbg!(&new_env_resource.get_kind());
    dbg!(&new_env_resource.get_coordinates());
    dbg!(&new_env_resource);
}
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
    let too_far_location = Coordinates::new(999,-999);
    let good_location = Coordinates::new(100, 100);
    zeus.display_info();
    zeus.display_resources();
    zeus.to_location(&too_far_location);
    zeus.to_location(&good_location);
    zeus.display_info();
    zeus.display_resources();
}
