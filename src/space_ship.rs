#![warn(missing_docs)]
use crate::mother_ship::MotherShip;
use crate::{
    GenericInfo, LevelCap, MotherShipDockStatus, MotherShipRechargeStatus, Resources,
    SpaceShipDockStatus, TranserResources, Coordinates, Move, EnvResource
};
use rand::{self, prelude::*};
use std::thread::sleep;
use std::time::Duration;
/// Struct for spaceships.
#[derive(Debug)]
pub struct SpaceShip<'a> {
    name: &'a str,
    consumables: Resources,
    oxygen: Resources,
    fuel: Resources,
    dock_status: SpaceShipDockStatus,
    location: Coordinates,
}
impl<'a> SpaceShip<'a> {
    fn docked(&mut self, mtr_shp: &mut MotherShip) {
        mtr_shp.change_status(
            Some(MotherShipDockStatus::Populated),
            Some(MotherShipRechargeStatus::Charging),
        );
        self.dock_status = SpaceShipDockStatus::Docked;
    }
    fn undocked(&mut self, mtr_shp: &mut MotherShip) {
        mtr_shp.change_status(
            Some(MotherShipDockStatus::Empty),
            Some(MotherShipRechargeStatus::Idle),
        );
        self.dock_status = SpaceShipDockStatus::Undocked;
    }
    fn recharge_backend(&mut self, mtr_shp: &mut MotherShip) {
        let initial_consumable_level = match self.consumables {
            Resources::FoodWater(val) => val,
            _ => 0,
        };
        let initial_oxygen_level = match self.oxygen {
            Resources::Oxygen(val) => val,
            _ => 0,
        };
        let initial_fuel_level = match self.fuel {
            Resources::Fuel(val) => val,
            _ => 0,
        };
        let a = [
            initial_fuel_level,
            initial_oxygen_level,
            initial_consumable_level,
        ];
        let min = a.iter().min().unwrap_or(&0);
        for _ in *min..100 {
            self.receive_resources(Resources::Fuel(1), mtr_shp);
            self.receive_resources(Resources::Oxygen(1), mtr_shp);
            self.receive_resources(Resources::FoodWater(1), mtr_shp);
            sleep(Duration::from_millis(200));
            self.display_resources();
        }
    }
    /// ## Creates a new spaceship
    /// Every spaceship is instantied with a name and is given a randomized resource starting values in range of 50 to 100.
    /// New spaceships are undocked, marked with SpaceShipDockStatus::Undocked
    /// ## Examples
    /// ```
    /// let mut zeus = SpaceShip::new("Zeus");
    /// ```
    pub fn new(n: &'a str) -> SpaceShip<'a> {
        let mut rng = rand::thread_rng();
        let mut s = SpaceShip {
            name: n,
            consumables: Resources::FoodWater(rng.gen_range(50..100)),
            oxygen: Resources::Oxygen(rng.gen_range(50..100)),
            fuel: Resources::Fuel(rng.gen_range(50..100)),
            dock_status: SpaceShipDockStatus::Undocked,
            location: Coordinates(rng.gen_range(5..100), rng.gen_range(5..100)),
        };
        s.consumables.adjust_spc_max_level();
        s.oxygen.adjust_spc_max_level();
        s.fuel.adjust_spc_max_level();
        s
    }
    /// Recharges a spaceship's resources.
    /// Resources will be taken from the mothership, decreasing their resource storage.
    /// ## Examples
    /// ```
    /// let mut ada = Mothership::new("Ada");
    /// let mut zeus = SpaceShip::new("Zeus");
    /// zeus.recharge(&mut ada);
    /// ```
    pub fn recharge(&mut self, mtr_shp: &mut MotherShip) {
        self.docked(mtr_shp);
        self.recharge_backend(mtr_shp);
        self.undocked(mtr_shp);
    }
}
impl<'a> TranserResources for SpaceShip<'a> {
    fn give_resources(&mut self, rsc: Resources, spc_current_level: i32) -> bool {
        match rsc {
            Resources::FoodWater(val) => {
                if spc_current_level - val > 0 {
                    self.consumables = Resources::FoodWater(spc_current_level - val);
                    true
                } else {
                    println!("Consumable unit to spend exceeds remaining.\nRemaining: {:?}\nNeeded: {val}", self.consumables);
                    false
                }
            }
            Resources::Oxygen(val) => {
                if spc_current_level - val > 0 {
                    self.oxygen = Resources::Oxygen(spc_current_level - val);
                    true
                } else {
                    println!("Oxygen unit to spend exceeds remaining.\nRemaining: {:?}\nNeeded: {val}", self.oxygen);
                    false
                }
            }
            Resources::Fuel(val) => {
                if spc_current_level - val > 0 {
                    self.fuel = Resources::Fuel(spc_current_level - val);
                    true
                } else {
                    println!("Fuel unit to spend exceeds remaining.\nRemaining: {:?}\nNeeded: {val}", self.fuel);
                    false
                }
            }
        }
    }
    fn receive_resources<T>(&mut self, rsc: Resources, mtr_shp: &mut T)
    where
        T: TranserResources,
    {
        match rsc {
            Resources::FoodWater(rate) => {
                let initial_consumable_level = match self.consumables {
                    Resources::FoodWater(val) => val,
                    _ => 0,
                };
                mtr_shp.give_resources(Resources::FoodWater(rate), initial_consumable_level);
                self.consumables = Resources::FoodWater(initial_consumable_level + rate);
                self.consumables.adjust_spc_max_level();
            }
            Resources::Oxygen(rate) => {
                let initial_oxygen_level = match self.oxygen {
                    Resources::Oxygen(val) => val,
                    _ => 0,
                };
                mtr_shp.give_resources(Resources::Oxygen(rate), initial_oxygen_level);
                self.oxygen = Resources::Oxygen(initial_oxygen_level + rate);
                self.oxygen.adjust_spc_max_level()
            }
            Resources::Fuel(rate) => {
                let initial_fuel_level = match self.fuel {
                    Resources::Fuel(val) => val,
                    _ => 0,
                };
                mtr_shp.give_resources(Resources::Fuel(rate), initial_fuel_level);
                self.fuel = Resources::Fuel(initial_fuel_level + rate);
                self.fuel.adjust_spc_max_level();
            }
        }
    }
    fn get_env_resources(&mut self, _env_resource: &mut EnvResource) {
    }
}
impl<'a> GenericInfo for SpaceShip<'a> {
    fn display_info(&self) {
        let n = self.name;
        println!("--{n}'s Information--\nCurrent location: ({},{})", self.location.0, self.location.1);
    }
    fn display_resources(&self) {
        let n = self.name;
        let c = match self.consumables {
            Resources::FoodWater(val) => val,
            _ => 0,
        };
        let o = match self.oxygen {
            Resources::Oxygen(val) => val,
            _ => 0,
        };
        let f = match self.fuel {
            Resources::Fuel(val) => val,
            _ => 0,
        };
        println!("--{n}'s Resources--\nFood & Water: {c}\nOxygen: {o}\nFuel: {f}");
    }
}
impl<'a> Move for SpaceShip<'a> {
    fn to_location(&mut self, to: &Coordinates) {
        let within_bounds = to.max_bounds();
        if within_bounds {
            let dist = to.get_distance(Coordinates::new(self.location.0, self.location.1));
            let fuel_to_spend = match dist {
                Some(val) => {
                    (val * 0.2).floor()
                }
                None => 0.0
            };
            let current_fuel = match self.fuel {
                Resources::Fuel(val) => val,
                _ => 0,
            };
            let enough_fuel = self.give_resources(Resources::Fuel(fuel_to_spend as i32), current_fuel);
            if enough_fuel {
                self.location.0 = to.0;
                self.location.1 = to.1;
                println!("Moved to ({}, {})", to.0, to.1);
            } else {
                println!("Not enough fuel to move to ({}, {})", to.0, to.1);
            }
        }
    }
}
