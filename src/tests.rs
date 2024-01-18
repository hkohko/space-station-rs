use std::collections::btree_map::Values;

use crate::prelude::*;
#[test]
fn transfer_storage() {
    let play_area: i32 = 100;
    let new_world = World::new(play_area, 500, 100, 1, 50, 1, 1, 0);
    let mut new_ship = SpaceShip::new("Zeus", &new_world);
    let mut env_1 = EnvResource::randomize(50, 0, new_world.play_area);
    let mut env_2 = EnvResource::randomize(100, 1, new_world.play_area);

    let status_env1 = new_ship.get_env_resources(&mut env_1);
    let status_env2 = new_ship.get_env_resources(&mut env_2);
    match status_env1 {
        GameWarning::Nominal => (),
        GameWarning::Unreachable => {
            assert!(new_ship.get_coordinates().get_distance(env_1.get_coordinates()) > 5.0);
        },
        GameWarning::ShipStorageFull => {
            match env_1.get_kind() {
                ResourceKind::Oxygen(val) => {
                    assert!(new_ship.get_storage().oxygen.0 + val > new_world.spaceship_storage_cap)
                }
                ResourceKind::Fuel(val) => {
                    assert!(new_ship.get_storage().fuel.0 + val > new_world.spaceship_storage_cap)
                }
                ResourceKind::FoodWater(val) => {
                    assert!(new_ship.get_storage().consumable.0 + val > new_world.spaceship_storage_cap)
                }
            }
        }
        GameWarning::ResourceExhausted => {
            match env_1.get_kind() {
                ResourceKind::FoodWater(val) => assert_eq!(val, 0),
                ResourceKind::Oxygen(val) => assert_eq!(val, 0),
                ResourceKind::Fuel(val) => assert_eq!(val, 0),
            }
        }
        _ => ()
    }
    match status_env2 {
        GameWarning::Nominal => (),
        GameWarning::Unreachable => {
            assert!(new_ship.get_coordinates().get_distance(env_1.get_coordinates()) > 5.0);
        },
        GameWarning::ShipStorageFull => {
            match env_2.get_kind() {
                ResourceKind::Oxygen(val) => {
                    assert!(new_ship.get_storage().oxygen.0 + val > new_world.spaceship_storage_cap)
                }
                ResourceKind::Fuel(val) => {
                    assert!(new_ship.get_storage().fuel.0 + val > new_world.spaceship_storage_cap)
                }
                ResourceKind::FoodWater(val) => {
                    assert!(new_ship.get_storage().consumable.0 + val > new_world.spaceship_storage_cap)
                }
            }
        }
        GameWarning::ResourceExhausted => {
            match env_2.get_kind() {
                ResourceKind::FoodWater(val) => assert_eq!(val, 0),
                ResourceKind::Oxygen(val) => assert_eq!(val, 0),
                ResourceKind::Fuel(val) => assert_eq!(val, 0),
            }
        }
        _ => ()
    }
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
#[test]
fn recharge_features() {
    let new_world = World::new(1000, 500, 100, 1, 200, 1, 1, 0);
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
