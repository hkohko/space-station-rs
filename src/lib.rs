use rand::{self, prelude::*};
pub trait GenericInfo{
    fn display_info(&self) {}
}
pub trait LevelCap {
    fn adjust_level(&mut self) {}
}
#[derive(Debug)]
pub struct MotherShip<'a> {
    pub name: &'a str,
    pub dock: bool,
    pub recharge: bool,
}
#[derive(Debug)]
pub struct SpaceShip<'a> {
    pub name: &'a str,
    pub consumables: FoodWater,
    pub oxygen: Oxygen,
    pub fuel: Fuel,
}
impl<'a> SpaceShip<'a> {
    pub fn new(n: &'a str) -> SpaceShip<'a> {
        let mut rng = rand::thread_rng();
        let mut s = SpaceShip {
            name: n,
            consumables: FoodWater::Level(rng.gen_range(50..100) as f32 * 1.1),
            oxygen: Oxygen::Level(rng.gen_range(50..100) as f32 * 1.2),
            fuel: Fuel::Level(rng.gen_range(50..100) as f32 * 1.1)
        };
        s.consumables.adjust_level();
        s.oxygen.adjust_level();
        s.fuel.adjust_level();
        s
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
pub enum Name<'a> {
    Name(&'a str)
}
#[derive(Debug)]
pub enum FoodWater{
    Level(f32),
}
#[derive(Debug)]
pub enum Oxygen {
    Level(f32),
}
#[derive(Debug)]
pub enum Fuel {
    Level(f32),
}
impl LevelCap for FoodWater {
    fn adjust_level(&mut self) {
        match self {
            Self::Level(val) => {
                *val = val.min(100.0);
            }
        };
    }
}
impl LevelCap for Oxygen {
    fn adjust_level(&mut self) {
        match self {
            Self::Level(val) => {
                *val = val.min(100.0)
            }
        }
    }
}
impl LevelCap for Fuel {
    fn adjust_level(&mut self) {
        match self {
            Self::Level(val) => {
                *val = val.min(100.0)
            }
        }
    }
}