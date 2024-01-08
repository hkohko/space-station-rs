pub mod mother_ship;
pub mod space_ship;

pub trait GenericInfo {
    fn display_info(&self) {}
    fn display_resources(&self) {}
}
pub trait LevelCap {
    fn adjust_spc_max_level(&mut self) {}
}
pub trait Recharge {
    fn recharge_consumables(&mut self, _rate: i32, _mtr_ship: &mut mother_ship::MotherShip) {}
    fn recharge_oxygen(&mut self, _rate: i32, _mtr_ship: &mut mother_ship::MotherShip) {}
    fn recharge_fuel(&mut self, _rate: i32, _mtr_ship: &mut mother_ship::MotherShip) {}
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
pub enum Resources {
    FoodWater(i32),
    Oxygen(i32),
    Fuel(i32),
}
impl LevelCap for Resources {
    fn adjust_spc_max_level(&mut self) {
        match self {
            Self::FoodWater(val) => {
                *val = std::cmp::min(*val, 100);
            }
            Self::Oxygen(val) => {
                *val = std::cmp::min(*val, 100);
            }
            Self::Fuel(val) => {
                *val = std::cmp::min(*val, 100);
            }
        };
    }
    
}
