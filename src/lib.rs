use rand::{self, prelude::*};
use std::thread::{current, sleep};
use std::time::Duration;

pub trait GenericInfo {
    fn display_info(&self) {}
}
pub trait LevelCap {
    fn adjust_level(&mut self) {}
}
pub trait SpaceShipRecharge {
    fn recharge_consumables(&mut self, rate: i32) {}
    fn recharge_oxygen(&mut self, rate: i32) {}
    fn recharge_fuel(&mut self, rate: i32) {}
}
#[derive(Debug)]
pub struct MotherShip<'a> {
    name: &'a str,
    dock: MotherShipDockStatus,
    recharge: MotherShipRechargeStatus,
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
        let mtr_ship_dock_msg: String;
        let mtr_ship_rchrg_msg: String;
        match self.dock {
            MotherShipDockStatus::Populated => {
                mtr_ship_dock_msg = String::from("A ship is docked.")
            }
            MotherShipDockStatus::Empty => mtr_ship_dock_msg = String::from("No ship is docked."),
        }
        match self.recharge {
            MotherShipRechargeStatus::Charging => {
                mtr_ship_rchrg_msg = String::from("Recharging a ship")
            }
            MotherShipRechargeStatus::Idle => {
                mtr_ship_rchrg_msg = String::from("Recharge port is vacant")
            }
        }
        println!("--Mothership Status--\nName: {}\nDock Status: {mtr_ship_dock_msg}\nRecharge Status: {mtr_ship_rchrg_msg}", self.name);
    }
}
#[derive(Debug)]
pub struct SpaceShip<'a> {
    name: &'a str,
    consumables: FoodWater,
    oxygen: Oxygen,
    fuel: Fuel,
    dock_status: SpaceShipDockStatus,
}
impl<'a> SpaceShip<'a> {
    pub fn new(n: &'a str) -> SpaceShip<'a> {
        let mut rng = rand::thread_rng();
        let mut s = SpaceShip {
            name: n,
            consumables: FoodWater::Level(rng.gen_range(50..100)),
            oxygen: Oxygen::Level(rng.gen_range(50..100)),
            fuel: Fuel::Level(rng.gen_range(50..100)),
            dock_status: SpaceShipDockStatus::Undocked,
        };
        s.consumables.adjust_level();
        s.oxygen.adjust_level();
        s.fuel.adjust_level();
        s
    }
    pub fn recharge(&mut self, mtr_shp: &mut MotherShip) {
        mtr_shp.dock = MotherShipDockStatus::Populated;
        mtr_shp.recharge = MotherShipRechargeStatus::Charging;
        self.dock_status = SpaceShipDockStatus::Docked;

        let initial_consumable_level = match self.consumables {
            FoodWater::Level(val) => val,
        };
        let initial_oxygen_level = match self.oxygen {
            Oxygen::Level(val) => val,
        };
        let initial_fuel_level = match self.fuel {
            Fuel::Level(val) => val,
        };
        let a = [
            initial_fuel_level,
            initial_oxygen_level,
            initial_consumable_level,
        ];
        let min = a.iter().min().unwrap_or(&0);
        mtr_shp.display_info();
        for _ in *min..100 {
            self.recharge_consumables(1);
            self.recharge_oxygen(1);
            self.recharge_fuel(1);
            sleep(Duration::from_millis(400));
            self.display_info();
        }
        mtr_shp.dock = MotherShipDockStatus::Empty;
        mtr_shp.recharge = MotherShipRechargeStatus::Idle;
        self.dock_status = SpaceShipDockStatus::Undocked;
    }
}
impl<'a> SpaceShipRecharge for SpaceShip<'a> {
    fn recharge_consumables(&mut self, rate: i32) {
        let initial_consumable_level = match self.consumables {
            FoodWater::Level(val) => val,
        };
        self.consumables = FoodWater::Level(initial_consumable_level + rate);
        self.consumables.adjust_level();
    }
    fn recharge_oxygen(&mut self, rate: i32) {
        let initial_oxygen_level = match self.oxygen {
            Oxygen::Level(val) => val,
        };
        self.oxygen = Oxygen::Level(initial_oxygen_level + rate);
        self.oxygen.adjust_level()
    }
    fn recharge_fuel(&mut self, rate: i32) {
        let initial_fuel_level = match self.fuel {
            Fuel::Level(val) => val,
        };
        self.fuel = Fuel::Level(initial_fuel_level + rate);
        self.fuel.adjust_level();
    }
}
impl<'a> GenericInfo for SpaceShip<'a> {
    fn display_info(&self) {
        let n = self.name;
        let c = match self.consumables {
            FoodWater::Level(val) => val,
        };
        let o = match self.oxygen {
            Oxygen::Level(val) => val,
        };
        let f = match self.fuel {
            Fuel::Level(val) => val,
        };
        println!("--Ship Status--\nName: {n}\nFood & Water: {c}\nOxygen: {o}\nFuel: {f}");
    }
}
#[derive(Debug)]
pub enum SpaceShipDockStatus {
    Docked,
    Undocked,
}
#[derive(Debug)]
pub enum MotherShipRechargeStatus {
    Charging,
    Idle,
}
#[derive(Debug)]
pub enum MotherShipDockStatus {
    Populated,
    Empty,
}
#[derive(Debug)]
pub enum Name<'a> {
    Name(&'a str),
}
#[derive(Debug)]
pub enum FoodWater {
    Level(i32),
}
#[derive(Debug)]
pub enum Oxygen {
    Level(i32),
}
#[derive(Debug)]
pub enum Fuel {
    Level(i32),
}
#[derive(Debug)]
pub enum RechargeFor {
    FoodWater(i32),
    Oxygen(i32),
    Fuel(i32),
}
impl LevelCap for FoodWater {
    fn adjust_level(&mut self) {
        match self {
            Self::Level(val) => {
                *val = std::cmp::min(*val, 100);
            }
        };
    }
}
impl LevelCap for Oxygen {
    fn adjust_level(&mut self) {
        match self {
            Self::Level(val) => {
                *val = std::cmp::min(*val, 100);
            }
        }
    }
}
impl LevelCap for Fuel {
    fn adjust_level(&mut self) {
        match self {
            Self::Level(val) => {
                *val = std::cmp::min(*val, 100);
            }
        }
    }
}
