use crate::prelude::*;
#[test]
fn transfer_storage() {
    let mut new_ship = SpaceShip::new("Zeus");
    let mut new_env_resources = EnvResource::randomize(50);
    dbg!(&new_ship);
    dbg!(&new_env_resources.get_kind());
    new_ship.get_env_resources(&mut new_env_resources);
    dbg!(&new_ship);
    dbg!(&new_env_resources.get_kind());
}
#[test]
fn storage_negative() {
    let mut new_stg = Storage::new(-10);
    new_stg.adjust_min_level();
    let consumables = new_stg.get_levels(Resources::FoodWater(0));
    let oxygen = new_stg.get_levels(Resources::Oxygen(0));
    let fuel = new_stg.get_levels(Resources::Fuel(0));
    assert_eq!(consumables, 0);
    assert_eq!(oxygen, 0);
    assert_eq!(fuel, 0);
}
#[test]
fn storage_positive() {
    let mut new_stg = Storage::new(100);
    new_stg.adjust_min_level();
    let consumables = new_stg.get_levels(Resources::FoodWater(0));
    let oxygen = new_stg.get_levels(Resources::Oxygen(0));
    let fuel = new_stg.get_levels(Resources::Fuel(0));
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
    let new_env_resource = EnvResource::randomize(50);
    dbg!(&new_env_resource.get_kind());
    dbg!(&new_env_resource.get_coordinates());
    dbg!(&new_env_resource);
}
#[test]
fn recharge_features() {
    let mut zeus = SpaceShip::new("Zeus");
    let mut ada = MotherShip::new("Ada");
    ada.display_resources();
    zeus.recharge(&mut ada, 0);
    let after_consumables = zeus.get_levels(Resources::Fuel(0));
    let after_oxygen = zeus.get_levels(Resources::Oxygen(0));
    let after_fuel = zeus.get_levels(Resources::Fuel(0));
    ada.display_resources();
    let diff_consumables = 100 - after_consumables;
    let diff_oxygen = 100 - after_oxygen;
    let diff_fuel = 100 - after_fuel;
    assert_eq!(diff_consumables, 0);
    assert_eq!(diff_fuel, 0);
    assert_eq!(diff_oxygen, 0);
}
#[test]
fn move_features() {
    let mut zeus = SpaceShip::new("Zeus");
    let too_far_location = Coordinates::new(999,-999);
    let good_location = Coordinates::new(100, 100);
    zeus.display_info();
    zeus.display_resources();
    let must_fail = zeus.to_location(&too_far_location);
    let must_pass = zeus.to_location(&good_location);
    zeus.display_info();
    zeus.display_resources();
    assert_ne!(must_fail, true);
    assert_eq!(must_pass, true)
}