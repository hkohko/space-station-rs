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
}
impl<'a> TransferResources for MotherShip<'a> {
    fn give_resources(&mut self, rsc: Resources, spc_current_level: i32) -> bool {
        let rate = self.world_parameters.consumption_rate;
        if spc_current_level == 100 {
            return false;
        }
        match rsc {
            Resources::FoodWater(_) => {
                if let Resources::Oxygen(val) = self.resource.consumable {
                    self.resource.consumable = Resources::Oxygen(val - rate);
                    return true;
                }
            }
            Resources::Oxygen(_) => {
                if let Resources::Oxygen(val) = self.resource.oxygen {
                    self.resource.oxygen = Resources::Oxygen(val - rate);
                    return true;
                }
            }
            Resources::Fuel(_) => {
                if let Resources::Fuel(val) = self.resource.fuel {
                    self.resource.fuel = Resources::Fuel(val - rate);
                    return true;
                }
            }
        }
        false
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
        let consumables = {
            if let Resources::Oxygen(val) = self.resource.consumable {
                val
            } else {
                0
            }
        };
        let oxygen = {
            if let Resources::Oxygen(val) = self.resource.oxygen {
                val
            } else {
                0
            }
        };
        let fuel = {
            if let Resources::Fuel(val) = self.resource.fuel {
                val
            } else {
                0
            }
        };
        println!("--Mothership '{}' Resources--\nConsumables: {consumables}\nOxygen: {oxygen}\nFuel: {fuel}", self.name)
    }
}
#[derive(Debug)]
/// Struct for mothershp resources.
struct MotherShipResource {
    consumable: Resources,
    oxygen: Resources,
    fuel: Resources,
}
impl MotherShipResource {
    fn new(amount: i32) -> MotherShipResource {
        MotherShipResource {
            consumable: Resources::Oxygen(amount),
            oxygen: Resources::Oxygen(amount),
            fuel: Resources::Fuel(amount),
        }
    }
}
