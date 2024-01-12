use space_station::prelude::*;
fn main() {}
#[test]
fn storage_negative() {
    let mut new_stg = Storage::new(-10);
    new_stg.adjust_min_level();
    let consumables = new_stg.get_stg_values(Resources::FoodWater(0));
    let oxygen = new_stg.get_stg_values(Resources::Oxygen(0));
    let fuel = new_stg.get_stg_values(Resources::Fuel(0));
    assert_eq!(consumables, 0);
    assert_eq!(oxygen, 0);
    assert_eq!(fuel, 0);
}
#[test]
fn storage_positive() {
    let mut new_stg = Storage::new(100);
    new_stg.adjust_min_level();
    let consumables = new_stg.get_stg_values(Resources::FoodWater(0));
    let oxygen = new_stg.get_stg_values(Resources::Oxygen(0));
    let fuel = new_stg.get_stg_values(Resources::Fuel(0));
    assert_eq!(consumables, 100);
    assert_eq!(oxygen, 100);
    assert_eq!(fuel, 100);
}
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
