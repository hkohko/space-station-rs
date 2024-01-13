use crate::prelude::*;
#[derive(Debug)]
pub struct World {
    spawned_resources: Vec<EnvResource>,
    consumption_rate: i32,
    recharge_rate: i32,
    game_tick: u8,
}
impl World {
    pub fn new(spawn_amount_of_resources: usize, resource_max_cap: i32, world_consumption_rate: i32, tick: u8, world_recharge_rate: i32) -> World {
        World {
            spawned_resources: World::randomize_resources(spawn_amount_of_resources, resource_max_cap),
            consumption_rate: world_consumption_rate,
            game_tick: tick,
            recharge_rate: world_recharge_rate,
        }
    }
    fn randomize_resources(amount: usize, at_most: i32) -> Vec<EnvResource>{
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
            rsc_vec.push(EnvResource::randomize(at_most, num))
        }
        rsc_vec
    }
    
}

