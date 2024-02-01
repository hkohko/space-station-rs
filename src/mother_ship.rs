#![warn(missing_docs)]
use crate::prelude::*;
/// Struct for motherships.
#[derive(Debug)]
pub struct MotherShip<'a> {
    name: &'a str,
    resource: MotherShipResource,
    dock: MotherShipDockStatus,
    recharge: MotherShipRechargeStatus,
    location: Coordinates,
    storage: Storage,
    world_parameters: &'a World,
}
impl<'a> MotherShip<'a> {
    /// ## Creates a new mothership
    /// A new mothership will:  
    /// - ...have 500 units for each resources,
    /// - ...have its dock status be MotherShipDockstatus::Empty,
    /// - ...have its recharge status be MotherShipRechargeStatus::Idle.
    /// ## Examples
    /// ```
    /// # use space_station::prelude::*;
    /// # use space_station::mother_ship::MotherShip;
    /// # let World = World::randomize(WorldSize::new(100));
    /// let mut ada = MotherShip::new("Ada", &World);
    /// ```
    pub fn new(n: &'a str, world: &'a World) -> MotherShip<'a> {
        let area = world.play_area;
        MotherShip {
            name: n,
            resource: MotherShipResource::new(500),
            dock: MotherShipDockStatus::Empty,
            recharge: MotherShipRechargeStatus::Idle,
            location: Coordinates::new(0, 0, area),
            storage: Storage::new(100),
            world_parameters: world,
        }
    }

    /// Change the dock and/or recharge status of the mothership.
    /// ## Examples
    /// ```
    /// # use space_station::prelude::*;
    /// # let World = World::randomize(WorldSize::new(100));
    /// let mut ada = MotherShip::new("Ada", &World);
    /// // Change ship's status to Populated.
    /// ada.change_status(Some(MotherShipDockStatus::Populated), None);
    /// ```
    pub fn change_status(
        &mut self,
        dock: Option<MotherShipDockStatus>,
        recharge: Option<MotherShipRechargeStatus>,
    ) {
        match dock {
            Some(val) => self.dock = val,
            None => (),
        };
        match recharge {
            Some(val) => self.recharge = val,
            None => (),
        };
    }
    /// Get Mothership's coordinates.
    pub fn get_coordinates(&self) -> Coordinates {
        self.location
    }
}
impl<'a> TransferResources for MotherShip<'a> {
    fn give_resources(&mut self, rsc: ResourceKind, spc_current_level: i32) -> bool {
        let rate = self.world_parameters.consumption_rate;
        if spc_current_level == 100 {
            return false;
        }
        match rsc {
            ResourceKind::FoodWater(_) => {
            self.resource.consumable = FoodWater(self.resource.consumable.0 - rate);
            return true;
                
            }
            ResourceKind::Oxygen(_) => {
                self.resource.oxygen = Oxygen(self.resource.oxygen.0 - rate);
                return true;
                
            }
            ResourceKind::Fuel(_) => {
                self.resource.fuel = Fuel(self.resource.fuel.0 - rate);
                return true;
            }
        }
    }
    fn receive_to_storage(&mut self, _rsc: ResourceKind) -> bool {
        let rate = self.world_parameters.recharge_rate;
        match _rsc {
            ResourceKind::FoodWater(_) => {
                let initial_consumable_level = self.storage.consumable.0;
                self.storage.consumable = FoodWater(initial_consumable_level + rate);
                true
            }
            ResourceKind::Oxygen(_) => {
                let initial_oxygen_level = self.storage.oxygen.0;
                self.storage.oxygen = Oxygen(initial_oxygen_level + rate);
                true
            }
            ResourceKind::Fuel(_) => {
                let initial_fuel_level = self.storage.fuel.0;
                self.storage.fuel = Fuel(initial_fuel_level + rate);
                true
            }
        }
    }
}
impl<'a> GenericInfo for MotherShip<'a> {
    fn display_info(&self) {
        let loc_x = self.location.x;
        let loc_y = self.location.y;
        let mtr_ship_dock_msg = match self.dock {
            MotherShipDockStatus::Populated => String::from("A ship is docked."),
            MotherShipDockStatus::Empty => String::from("No ship is docked."),
        };
        let mtr_ship_rchrg_msg = match self.recharge {
            MotherShipRechargeStatus::Charging => String::from("Recharging a ship"),
            MotherShipRechargeStatus::Idle => String::from("Recharge port is vacant"),
        };
        println!("--Mothership Status--\nName: {}\nLocation: {loc_x}, {loc_y}\nDock Status: {mtr_ship_dock_msg}\nRecharge Status: {mtr_ship_rchrg_msg}", self.name);
    }
    fn display_resources(&self) {
        let consumables = self.resource.consumable.0;
        let oxygen = self.resource.oxygen.0;
        let fuel = self.resource.fuel.0;
        println!("--Mothership '{}' Resources--\nConsumables: {consumables}\nOxygen: {oxygen}\nFuel: {fuel}", self.name)
    }
    fn display_storage(&self) {
        let consumables = self.storage.consumable.0;
        let oxygen = self.storage.oxygen.0;
        let fuel = self.storage.fuel.0;
        println!("--Mothership '{}' Storage--\nConsumables: {consumables}\nOxygen: {oxygen}\nFuel: {fuel}", self.name);
    }
}
#[derive(Debug)]
/// Struct for mothershp resources.
struct MotherShipResource {
    consumable: FoodWater,
    oxygen: Oxygen,
    fuel: Fuel,
}
impl MotherShipResource {
    fn new(amount: i32) -> MotherShipResource {
        MotherShipResource {
            consumable: FoodWater(amount),
            oxygen: Oxygen(amount),
            fuel: Fuel(amount),
        }
    }
}
