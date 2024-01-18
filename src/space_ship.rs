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
    consumable: FoodWater,
    oxygen: Oxygen,
    fuel: Fuel,
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
        let recharge_amount_world_param = self.world_parameters.recharge_rate;
        let initial_consumable_level = self.consumable.0;
        let initial_oxygen_level = self.oxygen.0;
        let initial_fuel_level = self.fuel.0;
        let a = [
            initial_fuel_level,
            initial_oxygen_level,
            initial_consumable_level,
        ];
        let min = a.iter().min().unwrap_or(&0);
        for _ in *min..100 {
            self.receive_resources(ResourceKind::Fuel(recharge_amount_world_param), mtr_shp);
            self.receive_resources(ResourceKind::Oxygen(recharge_amount_world_param), mtr_shp);
            self.receive_resources(ResourceKind::FoodWater(recharge_amount_world_param), mtr_shp);
            sleep(Duration::from_millis(recharge_ms));
        }
    }
    /// ## Creates a new spaceship
    /// Every spaceship is instantied with a name and is given a randomized resource starting values in range of 50 to 100.
    /// New spaceships are undocked, marked with SpaceShipDockStatus::Undocked
    /// ## Examples
    /// ```
    /// # use space_station::prelude::*;
    /// # let World = World::randomize(WorldSize::new(100));
    /// let mut zeus = SpaceShip::new("Zeus", &World);
    /// ```
    pub fn new(n: &'a str, world: &'a World) -> SpaceShip<'a> {
        let mut rng = rand::thread_rng();

        SpaceShip {
            name: n,
            consumable: FoodWater(rng.gen_range(50..100)),
            oxygen: Oxygen(rng.gen_range(50..100)),
            fuel: Fuel(rng.gen_range(50..100)),
            dock_status: SpaceShipDockStatus::Undocked,
            location: Coordinates::randomize(world.play_area),
            storage: Storage::new(0),
            world_parameters: world,
        }
    }
    /// Recharges a spaceship's resources.
    /// Resources will be taken from the mothership, decreasing their resource storage.
    /// ## Examples
    /// ```
    /// # use space_station::prelude::*;
    /// # let World = World::randomize(WorldSize::new(100));
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
    /// Shows resources within a certain distance of the ship.
    pub fn ping(&self) {
        println!(
            "{0: <20}  |  {1: <20}  |  {2: <20}",
            "ID", "Resource", "Location\n"
        );
        let resource = &self.world_parameters.spawned_resources;
        for refcell_rsc in resource.iter() {
            let rsc = refcell_rsc.borrow();
            let resource_location = rsc.get_coordinates().get_distance(self.location);
            if resource_location < 120.0 {
                let kind = rsc.get_kind();
                let x_axis = rsc.get_coordinates().x;
                let y_axis = rsc.get_coordinates().y;
                let id = rsc.get_id();
                let axis_string = format!("{x_axis}, {y_axis}");
                let kind_string = format!("{kind:?}");
                println!(
                    "{0: <20}  |  {1: <20}  |  {2: <20}",
                    id, kind_string, axis_string
                );
            }
        }
    }
    /// Return the location of the spaceship.
    pub fn get_location(&self) -> (i32, i32) {
        (self.location.x, self.location.y)
    }
}
impl<'a> TransferResources for SpaceShip<'a> {
    fn give_resources(&mut self, rsc: ResourceKind, spc_current_level: i32) -> bool {
        match rsc {
            ResourceKind::FoodWater(val) => {
                if spc_current_level - val > 0 {
                    self.consumable = FoodWater(spc_current_level - val);
                    true
                } else {
                    println!("Consumable unit to spend exceeds remaining.\nRemaining: {:?}\nNeeded: {val}", self.consumable);
                    false
                }
            }
            ResourceKind::Oxygen(val) => {
                if spc_current_level - val > 0 {
                    self.oxygen = Oxygen(spc_current_level - val);
                    true
                } else {
                    println!(
                        "Oxygen unit to spend exceeds remaining.\nRemaining: {:?}\nNeeded: {val}",
                        self.oxygen
                    );
                    false
                }
            }
            ResourceKind::Fuel(val) => {
                if spc_current_level - val > 0 {
                    self.fuel = Fuel(spc_current_level - val);
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
    fn receive_resources<T>(&mut self, rsc: ResourceKind, source: &mut T)
    where
        T: TransferResources,
    {
        let rate = self.world_parameters.recharge_rate;
        match rsc {
            ResourceKind::FoodWater(_) => {
                let initial_consumable_level = self.consumable.0;
                let still_available =
                    source.give_resources(ResourceKind::FoodWater(0), initial_consumable_level);
                if still_available {
                    self.consumable = FoodWater(initial_consumable_level + rate);
                    self.consumable.adjust_max_level();
                }
            }
            ResourceKind::Oxygen(_) => {
                let initial_oxygen_level = self.oxygen.0;
                let still_available =
                    source.give_resources(ResourceKind::Oxygen(0), initial_oxygen_level);
                if still_available {
                    self.oxygen = Oxygen(initial_oxygen_level + rate);
                    self.oxygen.adjust_max_level()
                }
            }
            ResourceKind::Fuel(_) => {
                let initial_fuel_level = self.fuel.0;
                let still_available = source.give_resources(ResourceKind::Fuel(0), initial_fuel_level);
                if still_available {
                    self.fuel = Fuel(initial_fuel_level + rate);
                    self.fuel.adjust_max_level();
                }
            }
        }
    }
    fn receive_to_storage(&mut self, _rsc: ResourceKind) -> bool {
        let max_cap = 50;
        match _rsc {
            ResourceKind::FoodWater(rate) => {
                if self.storage.consumable.0 == max_cap {
                    println!("FoodWater storage is full!");
                    return true 
                }
                self.storage.consumable = FoodWater(self.storage.consumable.0 + rate);
                false
            }
            ResourceKind::Oxygen(rate) => {
                if self.storage.oxygen.0 == max_cap {
                    println!("Oxygen storage is full!");
                    return true
                }
                self.storage.oxygen = Oxygen(self.storage.oxygen.0 + rate);
                false
            }
            ResourceKind::Fuel(rate) => {
                if self.storage.fuel.0 == max_cap {
                    println!("Fuel storage is full!");
                    return true
                }
                self.storage.fuel = Fuel(self.storage.fuel.0 + rate);
                false
            }
        }
    }
    fn get_env_resources(&mut self, _env_resource: &mut EnvResource) {
        if !(self.location.get_distance(_env_resource.get_coordinates()) < 5.0) {
            println!("You are too far from the target resource");
            return;
        }
        let recharge_rate = self.world_parameters.recharge_rate;
        let consumption_rate = self.world_parameters.consumption_rate;
        match _env_resource.get_kind() {
            ResourceKind::FoodWater(val) => {
                for _ in 0..=val {
                    let check_full = self.receive_to_storage(ResourceKind::FoodWater(0));
                    if check_full {
                        break
                    }
                    let still_available =
                        _env_resource.give_resources(ResourceKind::FoodWater(consumption_rate), val);
                    if still_available {
                        let is_full = self.receive_to_storage(ResourceKind::FoodWater(recharge_rate));
                        if is_full {
                            break
                        }
                    } else {
                        break
                    }
                }
            }
            ResourceKind::Oxygen(val) => {
                for _ in 0..=val {
                    let check_full = self.receive_to_storage(ResourceKind::Oxygen(0));
                    if check_full {
                        break
                    }
                    let still_available = _env_resource.give_resources(ResourceKind::Oxygen(consumption_rate), val);
                    if still_available {
                        let is_full = self.receive_to_storage(ResourceKind::Oxygen(recharge_rate));
                        if is_full {                   
                            break
                        }
                    } else {
                        break
                    }
                }
            }
            ResourceKind::Fuel(val) => {
                for _ in 0..=val {
                    let check_full = self.receive_to_storage(ResourceKind::Fuel(0));
                    if check_full {
                        break
                    }
                    let still_available = _env_resource.give_resources(ResourceKind::Fuel(consumption_rate), val);
                    if still_available {
                        let is_full = self.receive_to_storage(ResourceKind::Fuel(recharge_rate));
                        if is_full {                 
                            break
                        }
                    } else {
                        break
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
            self.location.x, self.location.y
        );
    }
    fn display_resources(&self) {
        let n = self.name;
        let c = self.consumable.0;
        let o = self.oxygen.0;
        let f = self.fuel.0;
        println!("--{n}'s Resources--\nFood & Water: {c}\nOxygen: {o}\nFuel: {f}");
    }
    fn display_storage(&self) {
        let consumable = self.storage.consumable.0;
        let oxygen = self.storage.oxygen.0;
        let fuel = self.storage.fuel.0;
        println!(
            "--{}'s Storage--\nFoodWater: {consumable}\nOxygen: {oxygen}\nFuel: {fuel}",
            self.name
        );
    }
}
impl<'a> Move for SpaceShip<'a> {
    fn to_location(&mut self, to: Coordinates) -> bool {
        let within_bounds = to.max_bounds();
        if within_bounds {
            let dist = to.get_distance(Coordinates::new(
                self.location.x,
                self.location.y,
                self.world_parameters.play_area,
            ));
            let fuel_to_spend = (dist * 0.2).floor();
            let current_fuel = self.fuel.0;
            let enough_fuel =
                self.give_resources(ResourceKind::Fuel(fuel_to_spend as i32), current_fuel);
            if enough_fuel {
                self.location.x = to.x;
                self.location.y = to.y;
                println!("Moved to ({}, {})", to.x, to.y);
                true
            } else {
                println!("Not enough fuel to move to ({}, {})", to.x, to.y);
                false
            }
        } else {
            false
        }
    }
    fn teleport(&mut self, mtr_ship: &MotherShip) {
        let mtr_ship_loc_x = mtr_ship.get_coordinates().x;
        let mtr_ship_loc_y = mtr_ship.get_coordinates().y;
        self.location = Coordinates::new(
            mtr_ship_loc_x,
            mtr_ship_loc_y,
            self.world_parameters.play_area,
        );
    }
}
impl<'a> GetResourceLevels for SpaceShip<'a> {
    fn get_resource_amount(&self, rsc: ResourceKind) -> i32 {
        match rsc {
            ResourceKind::Fuel(_) => {
                self.fuel.0
            }
            ResourceKind::Oxygen(_) => {
                self.oxygen.0
            }
            ResourceKind::FoodWater(_) => {
                self.consumable.0
            }
        }
    }
}
