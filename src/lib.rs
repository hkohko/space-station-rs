// #![warn(missing_docs)]
//! # space-station-rs
//!
//! Exploring Rust's type system by creating a space station game.
//!
//! Inspired by [No Boilerplate's](https://www.youtube.com/@NoBoilerplate) video on youtube:
//! ['Building a space station in Rust'](https://www.youtube.com/watch?v=7GzQArrek7A&pp=ygUdbm8gYm9pbGVycGxhdGUgcnVzdCBzcGFjZXNoaXA%3D)
//!

/// Structs, Enums, and methods for motherships.
pub mod mother_ship;
/// Structs, Enums, and methods for spaceships.
pub mod space_ship;
/// Shared trait for generic information of a ship.
pub trait GenericInfo {
    /// Displays a ship's general information.
    fn display_info(&self) {}
    /// Displays a ship's current resources.
    fn display_resources(&self) {}
}
/// Shared trait for mother-type ships.
pub trait isMother {}
/// Implements a level cap on resources for ships.
pub trait LevelCap {
    /// Limit the max level for resources on a spaceship.
    fn adjust_spc_max_level(&mut self) {}
}
/// Shared trait for ships that can transfer resources, be it receiving or giving.
pub trait TranserResources {
    /// Modify resources currently available on the mothership.
    /// ## Examples
    /// ```
    /// let mut ada = MotherShip::new("Ada");
    /// ada.give_resources(Resources::FoodWater, 1, spc_current_level);
    /// ```
    fn give_resources(&mut self, _rsc: Resources, _current_level: i32) {}
    fn receive_resources<T>(&mut self, _rsc: Resources, _mtr_shp: &mut T) where T: TranserResources {}
}

/// Shared trait for recharging resources.
pub trait ReceiveResources {
    /// Recharge consumables.
    fn receive_consumables(&mut self, _rate: i32, _mtr_ship: &mut mother_ship::MotherShip) {}
    /// Recharge oxygen.
    fn receive_oxygen(&mut self, _rate: i32, _mtr_ship: &mut mother_ship::MotherShip) {}
    /// Recharge fuel.
    fn receive_fuel(&mut self, _rate: i32, _mtr_ship: &mut mother_ship::MotherShip) {}
}
/// Spaceship docking enums.
#[derive(Debug)]
pub enum SpaceShipDockStatus {
    /// Spaceship is docked.
    Docked,
    /// Spaceship is undocked.
    Undocked,
}
/// Mothership recharge enums.
#[derive(Debug)]
pub enum MotherShipRechargeStatus {
    /// Mothership is recharging a spaceship.
    Charging,
    /// Charging port is vacant.
    Idle,
}
/// Mothership docking enums.
#[derive(Debug)]
pub enum MotherShipDockStatus {
    /// A spaceship is currently docked on the mothership.
    Populated,
    /// Mothership docking area is empty.
    Empty,
}
/// Deprecated enums for ship names in favor of simple &str.
#[derive(Debug)]
pub enum Name<'a> {
    /// Contains a ship's name. Takes &str.
    Name(&'a str),
}
/// The main resources of the game.
#[derive(Debug)]
pub enum Resources {
    /// Consumables.
    FoodWater(i32),
    /// Breathable air.
    Oxygen(i32),
    /// Rocket fuel.
    Fuel(i32),
}
impl LevelCap for Resources {
    fn adjust_spc_max_level(&mut self) {
        match self {
            Self::FoodWater(val) => {
                *val = std::cmp::min(*val, 100);
            }
            Self::Oxygen(val) => {
                *val = std::cmp::min(*val, 100);
            }
            Self::Fuel(val) => {
                *val = std::cmp::min(*val, 100);
            }
        };
    }
    
}
