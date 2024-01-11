use space_station::{mother_ship::MotherShip, space_ship::SpaceShip, GenericInfo, Move, Coordinates, Resources, environment_resources::EnvResource};
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
    let new_location = Coordinates::new(999,-999);
    zeus.display_info();
    zeus.display_resources();
    zeus.to_location(&new_location);
    zeus.display_info();
    zeus.display_resources();
}
