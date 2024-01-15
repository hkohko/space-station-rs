use crate::prelude::*;
#[test]
fn transfer_storage() {
    let play_area: i32 = 1000;
    let new_world = World::randomize(WorldSize::new(play_area));
    let mut new_ship = SpaceShip::new("Zeus", &new_world);
    let mut new_env_resources = EnvResource::randomize(50, 0, new_world.play_area);
    let mut env_resources2 = EnvResource::randomize(100, 1, new_world.play_area);
    dbg!(&new_ship);
    dbg!(&new_env_resources.get_kind());
    new_ship.get_env_resources(&mut new_env_resources);
    new_ship.get_env_resources(&mut env_resources2);
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
    let play_area: i32 = 1000;
    let w_size = WorldSize::new(play_area);
    let new_coord = Coordinates::randomize(w_size);
    dbg!(&new_coord);
    let new_resources = Resources::randomize(50);
    dbg!(&new_resources);
    let new_env_resource = EnvResource::randomize(50, 3, w_size);
    dbg!(&new_env_resource.get_kind());
    dbg!(&new_env_resource.get_coordinates());
    dbg!(&new_env_resource);
}
#[test]
fn recharge_features() {
    let new_world = World::randomize(WorldSize::new(1000));
    let mut zeus = SpaceShip::new("Zeus", &new_world);
    let mut ada = MotherShip::new("Ada", &new_world);
    ada.display_resources();
    zeus.recharge(&mut ada);
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
    let play_area = 1000;
    let new_world = World::randomize(WorldSize::new(play_area));
    let mut zeus = SpaceShip::new("Zeus", &new_world);
    let too_far_location = Coordinates::new(2 * play_area, -2 * play_area , new_world.play_area);
    let good_location = Coordinates::new(play_area / 2, play_area - 500, new_world.play_area);
    let (min, max) = new_world.play_area.get_values();
    zeus.display_info();
    zeus.display_resources();
    let must_fail = zeus.to_location(&too_far_location);
    let must_pass = zeus.to_location(&good_location);
    zeus.display_info();
    zeus.display_resources();
    assert_ne!(must_fail, true);
    assert_eq!(must_pass, true)
}
