pub mod mother_ship;
pub mod space_ship;

pub trait GenericInfo {
    fn display_info(&self) {}
}
pub trait LevelCap {
    fn adjust_level(&mut self) {}
}
pub trait SpaceShipRecharge {
    fn recharge_consumables(&mut self, _rate: i32) {}
    fn recharge_oxygen(&mut self, _rate: i32) {}
    fn recharge_fuel(&mut self, _rate: i32) {}
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
