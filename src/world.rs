use crate::prelude::*;
#[derive(Debug)]
pub struct World {
    spawned_resources: Vec<EnvResource>,
    consumption_rate: i32,
    game_tick: u8,
}

