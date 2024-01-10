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
/// Implements a level cap on resources for ships.
pub trait LevelCap {
    /// Limit the max level for resources on a spaceship.
    fn adjust_spc_max_level(&mut self) {}
    fn adjust_spc_min_level(&mut self) {}
}
/// Shared trait for ships that can transfer resources, be it receiving or giving.
pub trait TranserResources {
    /// Modify resources currently available on the mothership.
    /// ## Examples
    /// ```
    /// let mut ada = MotherShip::new("Ada");
    /// ada.give_resources(Resources::FoodWater(1), spc_current_level);
    /// ```
    fn give_resources(&mut self, _rsc: Resources, _current_level: i32) -> bool {true}
    fn receive_resources<T>(&mut self, _rsc: Resources, _mtr_shp: &mut T)
    where
        T: TranserResources,
    {
    }
}
pub trait Move {
    fn to_location(&mut self, _to: &Coordinates) {}
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
    fn adjust_spc_min_level(&mut self) {
        match self {
            Self::FoodWater(val) => {
                *val = std::cmp::max(*val, 0);
            },
            Self::Oxygen(val) => {
                *val = std::cmp::max(*val, 0);
            },
            Self::Fuel(val) => {
                *val = std::cmp::max(*val, 0);
            }
        }
    }
}
#[derive(Debug)]
pub struct Coordinates(i32, i32);
pub enum Quadrants {
    First,
    Second,
    Third,
    Fourth,
}
pub struct Location();

impl Coordinates {
    pub fn new(x: i32, y: i32) -> Self {
        Coordinates(x, y)
    }
    fn max_bounds(&self) -> bool {
        let mut is_valid = true;
        let values = [self.0, self.1];
        for item in values.into_iter().enumerate() {
            let (idx, val) = item;
            if val < -1000 || val > 1000 {
                is_valid = false;
                let axis = if idx == 0 {"x"} else {"y"};
                println!("{axis} value is out of bounds: {val}.")
            }
        }
        is_valid
    }
    fn get_quadrants(&self) -> Quadrants {
        if self.0.is_positive() && self.1.is_positive() {
            Quadrants::First
        } else if self.0.is_negative() && self.1.is_positive() {
            Quadrants::Second
        } else if self.0.is_negative() && self.1.is_negative() {
            Quadrants::Third
        } else {
            Quadrants::Fourth
        }
    }
    pub fn get_distance(&self, from: Coordinates) -> Option<f64> {
        let side_a = from.0 - self.0;
        let side_b = from.1 - self.1;
        let dest = f64::try_from(side_a.pow(2) + side_b.pow(2));
        let sqrt = match dest {
            Ok(val) => {
                let sqrt = val.sqrt().floor();
                Some(sqrt)
            },
            Err(e) => {
                println!("get_distance() Error: \n{e}");
                None
            },
        };
        sqrt
    }
}
