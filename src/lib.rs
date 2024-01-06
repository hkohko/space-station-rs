use rand::{self, prelude::*};

pub trait GenericInfo{
    fn display_info(&self) {}
}
#[derive(Debug)]
pub struct SpaceStation {
    pub consumables: FoodWater,
    pub oxygen: Oxygen,
    pub fuel: Fuel,
}
impl SpaceStation {
    pub fn new() -> SpaceStation{
        let mut rng = rand::thread_rng();
        SpaceStation {
            consumables: FoodWater::Level(rng.gen_range(50..100) as f32 * 1.1),
            oxygen: Oxygen::Level(rng.gen_range(50..100) as f32 * 1.2),
            fuel: Fuel::Level(rng.gen_range(50..100) as f32 * 1.1)
        }
    }
}
impl GenericInfo for SpaceStation {
    fn display_info(&self) {
        let c = match self.consumables {
            FoodWater::Level(val) => val,
        };
        let o = match self.oxygen {
            Oxygen::Level(val) => val,
        };
        let f = match self.fuel {
            Fuel::Level(val) => val,
        };
        println!("Your ship currently has these status:\nFood & Water: {c}\nOxygen: {o}\nFuel: {f}");
    }
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
