#![warn(missing_docs)]
use crate::{
    GenericInfo, MotherShipDockStatus, MotherShipRechargeStatus, Resources, TranserResources, Coordinates
};
/// Struct for motherships.
#[derive(Debug)]
pub struct MotherShip<'a> {
    name: &'a str,
    resource: MotherShipResource,
    dock: MotherShipDockStatus,
    recharge: MotherShipRechargeStatus,
    location: Coordinates
}
impl<'a> MotherShip<'a> {
    /// ## Creates a new mothership
    /// A new mothership will:  
    /// - ...have 500 units for each resources,
    /// - ...have its dock status be MotherShipDockstatus::Empty,
    /// - ...have its recharge status be MotherShipRechargeStatus::Idle.
    /// ## Examples
    /// ```
    /// let mut ada = MotherShip::new("Ada");
    /// ```
    pub fn new(n: &'a str) -> MotherShip<'a> {
        MotherShip {
            name: n,
            resource: MotherShipResource {
                consumable: Resources::FoodWater(500),
                oxygen: Resources::Oxygen(500),
                fuel: Resources::Fuel(500),
            },
            dock: MotherShipDockStatus::Empty,
            recharge: MotherShipRechargeStatus::Idle,
            location: Coordinates(0, 0),
        }
    }

    /// Change the dock and/or recharge status of the mothership.
    /// ## Examples
    /// ```
    /// let mut ada = MotherShip::new("Ada");
    /// // Change ship's status to Populated.
    /// ada.change_status(Some(MotherShipDocStatus::Populated), None);
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
impl<'a> TranserResources for MotherShip<'a> {
    fn give_resources(&mut self, rsc: Resources, spc_current_level: i32) -> bool {
        if spc_current_level == 100 {
            return false
        }
        match rsc {
            Resources::FoodWater(rate) => {
                if let Resources::FoodWater(val) = self.resource.consumable {
                    self.resource.consumable = Resources::FoodWater(val - rate);
                    return true
                }
            }
            Resources::Oxygen(rate) => {
                if let Resources::Oxygen(val) = self.resource.oxygen {
                    self.resource.oxygen = Resources::Oxygen(val - rate);
                    return true
                }
            }
            Resources::Fuel(rate) => {
                if let Resources::Fuel(val) = self.resource.fuel {
                    self.resource.fuel = Resources::Fuel(val - rate);
                    return true
                }
            }
        }
        false
    }
}
impl<'a> GenericInfo for MotherShip<'a> {
    fn display_info(&self) {
        let mtr_ship_dock_msg = match self.dock {
            MotherShipDockStatus::Populated => String::from("A ship is docked."),
            MotherShipDockStatus::Empty => String::from("No ship is docked."),
        };
        let mtr_ship_rchrg_msg = match self.recharge {
            MotherShipRechargeStatus::Charging => String::from("Recharging a ship"),
            MotherShipRechargeStatus::Idle => String::from("Recharge port is vacant"),
        };
        println!("--Mothership Status--\nName: {}\nDock Status: {mtr_ship_dock_msg}\nRecharge Status: {mtr_ship_rchrg_msg}", self.name);
    }
    fn display_resources(&self) {
        let consumables = {
            if let Resources::FoodWater(val) = self.resource.consumable {
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
