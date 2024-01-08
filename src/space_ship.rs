use crate::mother_ship::{MotherShip, self};
use crate::{
    Resources, GenericInfo, LevelCap, MotherShipDockStatus, MotherShipRechargeStatus,
    SpaceShipDockStatus, Recharge,
};
use rand::{self, prelude::*};
use std::thread::sleep;
use std::time::Duration;
#[derive(Debug)]
pub struct SpaceShip<'a> {
    name: &'a str,
    consumables: Resources,
    oxygen: Resources,
    fuel: Resources,
    dock_status: SpaceShipDockStatus,
}
impl<'a> SpaceShip<'a> {
    fn docked(&mut self, mtr_shp: &mut MotherShip) {
        mtr_shp.change_status(Some(MotherShipDockStatus::Populated), Some(MotherShipRechargeStatus::Charging));
        self.dock_status = SpaceShipDockStatus::Docked;
    }
    fn undocked(&mut self, mtr_shp: &mut MotherShip) {
        mtr_shp.change_status(Some(MotherShipDockStatus::Empty), Some(MotherShipRechargeStatus::Idle));
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
            self.recharge_consumables(1, mtr_shp);
            self.recharge_oxygen(1, mtr_shp);
            self.recharge_fuel(1, mtr_shp);
            sleep(Duration::from_millis(200));
            self.display_info();
        }
    }
    pub fn new(n: &'a str) -> SpaceShip<'a> {
        let mut rng = rand::thread_rng();
        let mut s = SpaceShip {
            name: n,
            consumables: Resources::FoodWater(rng.gen_range(50..100)),
            oxygen: Resources::Oxygen(rng.gen_range(50..100)),
            fuel: Resources::Fuel(rng.gen_range(50..100)),
            dock_status: SpaceShipDockStatus::Undocked,
        };
        s.consumables.adjust_spc_max_level();
        s.oxygen.adjust_spc_max_level();
        s.fuel.adjust_spc_max_level();
        s
    }
    pub fn recharge(&mut self, mtr_shp: &mut MotherShip) {
        self.docked(mtr_shp);
        self.recharge_backend(mtr_shp);
        self.undocked(mtr_shp);
    }
}
impl<'a> Recharge for SpaceShip<'a> {
    fn recharge_consumables(&mut self, rate: i32, mtr_shp: &mut MotherShip) {
        let initial_consumable_level =  match self.consumables {
            Resources::FoodWater(val) => val,
            _ => 0,
        };
        mtr_shp.modify_resources(Resources::FoodWater(0), rate, &initial_consumable_level);
        self.consumables = Resources::FoodWater(initial_consumable_level + rate);
        self.consumables.adjust_spc_max_level();
    }
    fn recharge_oxygen(&mut self, rate: i32, mtr_shp: &mut MotherShip) {
        let initial_oxygen_level = match self.oxygen {
            Resources::Oxygen(val) => val,
            _ => 0,
        };
        mtr_shp.modify_resources(Resources::Oxygen(0), rate, &initial_oxygen_level);
        self.oxygen = Resources::Oxygen(initial_oxygen_level + rate);
        self.oxygen.adjust_spc_max_level()
    }
    fn recharge_fuel(&mut self, rate: i32, mtr_shp: &mut MotherShip) {
        let initial_fuel_level = match self.fuel {
            Resources::Fuel(val) => val,
            _ => 0,
        };
        mtr_shp.modify_resources(Resources::Fuel(0), rate, &initial_fuel_level);
        self.fuel = Resources::Fuel(initial_fuel_level + rate);
        self.fuel.adjust_spc_max_level();
    }
}
impl<'a> GenericInfo for SpaceShip<'a> {
    fn display_info(&self) {
        let n = self.name;
        let c= match self.consumables {
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
        println!("--Ship Status--\nName: {n}\nFood & Water: {c}\nOxygen: {o}\nFuel: {f}");
    }
}
