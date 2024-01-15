#![warn(missing_docs)]
use crate::mother_ship::MotherShip;
use crate::prelude::*;
use rand::{self, prelude::*};
use std::thread::sleep;
use std::time::Duration;
/// Struct for spaceships.
#[derive(Debug)]
pub struct SpaceShip<'a> {
    name: &'a str,
    consumable: Resources,
    oxygen: Resources,
    fuel: Resources,
    storage: Storage,
    dock_status: SpaceShipDockStatus,
    location: Coordinates,
    world_parameters: &'a World,
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
    fn recharge_backend(&mut self, mtr_shp: &mut MotherShip, recharge_ms: u64) {
        let initial_consumable_level = match self.consumable {
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
            sleep(Duration::from_millis(recharge_ms));
        }
    }
    /// ## Creates a new spaceship
    /// Every spaceship is instantied with a name and is given a randomized resource starting values in range of 50 to 100.
    /// New spaceships are undocked, marked with SpaceShipDockStatus::Undocked
    /// ## Examples
    /// ```
    /// # use space_station::prelude::*;
    /// # let World = World::randomize();
    /// let mut zeus = SpaceShip::new("Zeus", &World);
    /// ```
    pub fn new(n: &'a str, world: &'a World) -> SpaceShip<'a> {
        let mut rng = rand::thread_rng();
        let play_area = world.play_area;
        let (min, max) = play_area.get_values();
        let s = SpaceShip {
            name: n,
            consumable: Resources::FoodWater(rng.gen_range(50..100)),
            oxygen: Resources::Oxygen(rng.gen_range(50..100)),
            fuel: Resources::Fuel(rng.gen_range(50..100)),
            dock_status: SpaceShipDockStatus::Undocked,
            location: Coordinates(rng.gen_range(min..max), rng.gen_range(min..max)),
            storage: Storage::new(0),
            world_parameters: world,
        };
        s
    }
    /// Recharges a spaceship's resources.
    /// Resources will be taken from the mothership, decreasing their resource storage.
    /// ## Examples
    /// ```
    /// # use space_station::prelude::*;
    /// # let World = World::randomize();
    /// let mut ada = MotherShip::new("Ada", &World);
    /// let mut zeus = SpaceShip::new("Zeus", &World);
    /// zeus.recharge(&mut ada);
    /// ```
    pub fn recharge(&mut self, mtr_shp: &mut MotherShip) {
        let recharge_ms = match u64::try_from(self.world_parameters.recharge_interval) {
            Ok(val) => val,
            Err(e) => {
                println!("{e}");
                0
            }
        };
        self.docked(mtr_shp);
        self.recharge_backend(mtr_shp, recharge_ms);
        self.undocked(mtr_shp);
    }
}
impl<'a> TransferResources for SpaceShip<'a> {
    fn give_resources(&mut self, rsc: Resources, spc_current_level: i32) -> bool {
        match rsc {
            Resources::FoodWater(val) => {
                if spc_current_level - val > 0 {
                    self.consumable = Resources::FoodWater(spc_current_level - val);
                    true
                } else {
                    println!("Consumable unit to spend exceeds remaining.\nRemaining: {:?}\nNeeded: {val}", self.consumable);
                    false
                }
            }
            Resources::Oxygen(val) => {
                if spc_current_level - val > 0 {
                    self.oxygen = Resources::Oxygen(spc_current_level - val);
                    true
                } else {
                    println!(
                        "Oxygen unit to spend exceeds remaining.\nRemaining: {:?}\nNeeded: {val}",
                        self.oxygen
                    );
                    false
                }
            }
            Resources::Fuel(val) => {
                if spc_current_level - val > 0 {
                    self.fuel = Resources::Fuel(spc_current_level - val);
                    true
                } else {
                    println!(
                        "Fuel unit to spend exceeds remaining.\nRemaining: {:?}\nNeeded: {val}",
                        self.fuel
                    );
                    false
                }
            }
        }
    }
    fn receive_resources<T>(&mut self, rsc: Resources, source: &mut T)
    where
        T: TransferResources,
    {
        let rate = self.world_parameters.recharge_rate;
        match rsc {
            Resources::FoodWater(_) => {
                let initial_consumable_level = match self.consumable {
                    Resources::FoodWater(val) => val,
                    _ => 0,
                };
                let still_available =
                    source.give_resources(Resources::FoodWater(0), initial_consumable_level);
                if still_available {
                    self.consumable = Resources::FoodWater(initial_consumable_level + rate);
                    self.consumable.adjust_max_level();
                }
            }
            Resources::Oxygen(_) => {
                let initial_oxygen_level = match self.oxygen {
                    Resources::Oxygen(val) => val,
                    _ => 0,
                };
                let still_available =
                    source.give_resources(Resources::Oxygen(0), initial_oxygen_level);
                if still_available {
                    self.oxygen = Resources::Oxygen(initial_oxygen_level + rate);
                    self.oxygen.adjust_max_level()
                }
            }
            Resources::Fuel(_) => {
                let initial_fuel_level = match self.fuel {
                    Resources::Fuel(val) => val,
                    _ => 0,
                };
                let still_available =
                    source.give_resources(Resources::Fuel(0), initial_fuel_level);
                if still_available {
                    self.fuel = Resources::Fuel(initial_fuel_level + rate);
                    self.fuel.adjust_max_level();
                }
            }
        }
    }
    fn receive_to_storage(&mut self, _rsc: Resources) {
        let rate = self.world_parameters.recharge_rate;
        match _rsc {
            Resources::FoodWater(_) => {
                let initial_consumable_level = match self.storage.consumable {
                    Resources::FoodWater(val) => val,
                    _ => 0,
                };
                self.storage.consumable = Resources::FoodWater(initial_consumable_level + rate);
            }
            Resources::Oxygen(_) => {
                let initial_oxygen_level = match self.storage.oxygen {
                    Resources::Oxygen(val) => val,
                    _ => 0,
                };
                self.storage.oxygen = Resources::Oxygen(initial_oxygen_level + rate);
            }
            Resources::Fuel(_) => {
                let initial_fuel_level = match self.storage.fuel {
                    Resources::Fuel(val) => val,
                    _ => 0,
                };
                self.storage.fuel = Resources::Fuel(initial_fuel_level + rate);
            }
        }
    }
    fn get_env_resources(&mut self, _env_resource: &mut EnvResource) {
        match _env_resource.get_kind() {
            Resources::FoodWater(val) => {
                for _ in 0..=val {
                    let still_available =
                        _env_resource.give_resources(Resources::FoodWater(1), val);
                    if still_available {
                        self.receive_to_storage(Resources::FoodWater(1))
                    }
                }
            }
            Resources::Oxygen(val) => {
                for _ in 0..=val {
                    let still_available = _env_resource.give_resources(Resources::Oxygen(1), val);
                    if still_available {
                        self.receive_to_storage(Resources::Oxygen(1))
                    }
                }
            }
            Resources::Fuel(val) => {
                for _ in 0..=val {
                    let still_available = _env_resource.give_resources(Resources::Fuel(1), val);
                    if still_available {
                        self.receive_to_storage(Resources::Fuel(1))
                    }
                }
            }
        }
    }
}
impl<'a> GenericInfo for SpaceShip<'a> {
    fn display_info(&self) {
        let n = self.name;
        println!(
            "--{n}'s Information--\nCurrent location: ({},{})",
            self.location.0, self.location.1
        );
    }
    fn display_resources(&self) {
        let n = self.name;
        let c = match self.consumable {
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
    fn to_location(&mut self, to: &Coordinates) -> bool {
        let within_bounds = to.max_bounds();
        if within_bounds {
            let dist = to.get_distance(Coordinates::new(self.location.0, self.location.1));
            let fuel_to_spend = (dist * 0.2).floor();
            let current_fuel = match self.fuel {
                Resources::Fuel(val) => val,
                _ => 0,
            };
            let enough_fuel =
                self.give_resources(Resources::Fuel(fuel_to_spend as i32), current_fuel);
            if enough_fuel {
                self.location.0 = to.0;
                self.location.1 = to.1;
                println!("Moved to ({}, {})", to.0, to.1);
                return true;
            } else {
                println!("Not enough fuel to move to ({}, {})", to.0, to.1);
                return false;
            }
        } else {
            false
        }
    }
}
impl<'a> GetResourceLevels for SpaceShip<'a> {
    fn get_levels(&self, rsc: Resources) -> i32 {
        match rsc {
            Resources::Fuel(_) => {
                if let Resources::Fuel(val) = self.fuel {
                    val
                } else {
                    0
                }
            }
            Resources::Oxygen(_) => {
                if let Resources::Oxygen(val) = self.oxygen {
                    val
                } else {
                    0
                }
            }
            Resources::FoodWater(_) => {
                if let Resources::FoodWater(val) = self.consumable {
                    val
                } else {
                    0
                }
            }
        }
    }
}
