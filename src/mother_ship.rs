use crate::{GenericInfo, MotherShipDockStatus, MotherShipRechargeStatus};
#[derive(Debug)]
pub struct MotherShip<'a> {
    name: &'a str,
    pub dock: MotherShipDockStatus,
    pub recharge: MotherShipRechargeStatus,
}
impl<'a> MotherShip<'a> {
    pub fn new(n: &'a str) -> MotherShip<'a> {
        MotherShip {
            name: n,
            dock: MotherShipDockStatus::Empty,
            recharge: MotherShipRechargeStatus::Idle,
        }
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
}
