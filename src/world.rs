use crate::prelude::*;
use rand::{self, Rng};
#[derive(Debug)]
pub struct World {
    pub spawned_resources: Vec<EnvResource>,
    pub consumption_rate: i32,
    pub recharge_interval: i32,
    pub recharge_rate: i32,
    pub game_tick: u8,
    pub play_area: WorldSize,
}
impl World {
    pub fn new(
        play_area: i32,
        spawn_amount_of_resources: usize,
        resource_max_cap: i32,
        world_consumption_rate: i32,
        tick: u8,
        world_recharge_rate: i32,
        world_recharge_interval: i32,
    ) -> World {
        let area = WorldSize::new(play_area);
        World {
            // problem: Coordinates inside envresource
            spawned_resources: EnvResource::randomize_resources(
                spawn_amount_of_resources,
                resource_max_cap,
                area,
            ),
            consumption_rate: world_consumption_rate,
            game_tick: tick,
            recharge_rate: world_recharge_rate,
            play_area: area,
            recharge_interval: world_recharge_interval,
        }
    }
    pub fn randomize(area: WorldSize) -> World {
        let mut rng = rand::thread_rng();
        World {
            spawned_resources: EnvResource::randomize_resources(100, 100, area),
            consumption_rate: 1,
            recharge_rate: 1,
            game_tick: rng.gen_range(1..5),
            play_area: area,
            recharge_interval: rng.gen_range(100..500),
        }
    }
    fn randomize_resources(amount: usize, at_most: i32, play_area: WorldSize) -> Vec<EnvResource> {
        let mut rsc_vec = Vec::with_capacity(amount);
        let convert_amount_to_i32 = i32::try_from(amount);
        let amount_as_i32 = match convert_amount_to_i32 {
            Ok(val) => val,
            Err(e) => {
                println!("Error converting world resource amount to i32\n\n{e}");
                0
            }
        };
        for num in 0..=amount_as_i32 {
            rsc_vec.push(EnvResource::randomize(at_most, num, play_area))
        }
        rsc_vec
    }
}
#[derive(Debug, Clone, Copy)]
pub struct WorldSize(i32, i32);
impl WorldSize {
    pub fn new(size: i32) -> WorldSize {
        WorldSize(-size, size)
    }
    pub fn randomize(min: i32, max: i32) -> WorldSize {
        let mut rng = rand::thread_rng();
        let max_range = rng.gen_range(min..max);
        WorldSize(-max_range, max_range)
    }
    pub fn get_values(&self) -> (i32, i32) {
        (self.0, self.1)
    }
}
