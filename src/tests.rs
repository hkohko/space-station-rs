use std::collections::btree_map::Values;

use crate::prelude::*;
// #[test]
fn transfer_storage() {
    let play_area: i32 = 2;
    let new_world = World::randomize(WorldSize::new(play_area));
    let mut new_ship = SpaceShip::new("Zeus", &new_world);
    let mut env_1 = EnvResource::randomize(50, 0, new_world.play_area);
    let mut env_2 = EnvResource::randomize(100, 1, new_world.play_area);
    let starting_env_1 = match env_1.get_kind() {
        ResourceKind::FoodWater(val) => val,
        ResourceKind::Fuel(val) => val,
        ResourceKind::Oxygen(val) => val,
    };
    let starting_env_2 = match env_2.get_kind() {
        ResourceKind::FoodWater(val) => val,
        ResourceKind::Fuel(val) => val,
        ResourceKind::Oxygen(val) => val,
    };
    new_ship.get_env_resources(&mut env_1);
    new_ship.get_env_resources(&mut env_2);
    let end_val_1 = match env_1.get_kind() {
        ResourceKind::FoodWater(val) => val,
        ResourceKind::Fuel(val) => val,
        ResourceKind::Oxygen(val) => val,
    };
    let end_val_2 = match env_2.get_kind() {
        ResourceKind::FoodWater(val) => val,
        ResourceKind::Fuel(val) => val,
        ResourceKind::Oxygen(val) => val,
    };
    // test needs to be modified to account for new features:
    // - distance
    // - level cap
    assert_eq!(end_val_1 - starting_env_1, -starting_env_1);
    assert_eq!(end_val_2 - starting_env_2, -starting_env_2)
}
#[test]
fn storage_negative() {
    let mut new_stg = Storage::new(-10);
    new_stg.adjust_min_level();
    let consumables = new_stg.get_resource_amount(ResourceKind::FoodWater(0));
    let oxygen = new_stg.get_resource_amount(ResourceKind::Oxygen(0));
    let fuel = new_stg.get_resource_amount(ResourceKind::Fuel(0));
    assert_eq!(consumables, 0);
    assert_eq!(oxygen, 0);
    assert_eq!(fuel, 0);
}
#[test]
fn storage_positive() {
    let mut new_stg = Storage::new(100);
    new_stg.adjust_min_level();
    let consumables = new_stg.get_resource_amount(ResourceKind::FoodWater(0));
    let oxygen = new_stg.get_resource_amount(ResourceKind::Oxygen(0));
    let fuel = new_stg.get_resource_amount(ResourceKind::Fuel(0));
    assert_eq!(consumables, 100);
    assert_eq!(oxygen, 100);
    assert_eq!(fuel, 100);
}
// #[test] irrelevant test
fn randomize_stuff() {
    let play_area: i32 = 1000;
    let w_size = WorldSize::new(play_area);
    let new_coord = Coordinates::randomize(w_size);
    dbg!(&new_coord);
    let new_resources = ResourceKind::randomize(50);
    dbg!(&new_resources);
    let new_env_resource = EnvResource::randomize(50, 3, w_size);
    dbg!(&new_env_resource.get_kind());
    dbg!(&new_env_resource.get_coordinates());
    dbg!(&new_env_resource);
}
#[test]
fn recharge_features() {
    let new_world = World::new(1000, 500, 100, 1, 1, 1, 0);
    let mut zeus = SpaceShip::new("Zeus", &new_world);
    let mut ada = MotherShip::new("Ada", &new_world);
    ada.display_resources();
    zeus.recharge(&mut ada);
    let after_consumables = zeus.get_resource_amount(ResourceKind::Fuel(0));
    let after_oxygen = zeus.get_resource_amount(ResourceKind::Oxygen(0));
    let after_fuel = zeus.get_resource_amount(ResourceKind::Fuel(0));
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
    let play_area = 120;
    let new_world = World::randomize(WorldSize::new(play_area));
    let mut zeus = SpaceShip::new("Zeus", &new_world);
    let too_far_location = Coordinates::new(2 * play_area, -2 * play_area, new_world.play_area);
    let good_location = Coordinates::new(play_area / 2, play_area / 2, new_world.play_area);
    zeus.display_info();
    zeus.display_resources();
    // take fuel reserves into account if this particular test fail
    // max distance = 100(1 + 0.2) <- 0.2x multiplier can be modified in space_ship.rs
    // impl Move <- here
    let must_fail = zeus.to_location(too_far_location);
    let must_pass = zeus.to_location(good_location);
    zeus.display_info();
    zeus.display_resources();
    assert_ne!(must_fail, true);
    assert_eq!(must_pass, true);
}
