use std::{cell::RefCell, rc::Rc};

use crate::prelude::*;
use rand::{self, Rng};
#[derive(Debug)]
/// Struct for a World. Every game must have a world.
pub struct World {
    /// Resources inside a world.
    pub spawned_resources: Vec<RefCell<EnvResource>>,
    /// Set consumption rate of ships.
    pub consumption_rate: i32,
    /// Set how fast a recharge should go.
    pub recharge_interval: i32,
    /// Set how much a game object is recharge per unit of time.
    pub recharge_rate: i32,
    /// Global game ticks.
    pub game_tick: u8,
    /// Set the play area of the World.
    pub play_area: WorldSize,
}
impl World {
    /// Creates a new world.
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
            spawned_resources: EnvResource::randomize_world_resources(
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
    /// Create a new World with randomized values.
    pub fn randomize(area: WorldSize) -> World {
        let mut rng = rand::thread_rng();
        World {
            spawned_resources: EnvResource::randomize_world_resources(100, 100, area),
            consumption_rate: 1,
            recharge_rate: 1,
            game_tick: rng.gen_range(1..5),
            play_area: area,
            recharge_interval: rng.gen_range(100..500),
        }
    }
}
/// Struct for WorldSize, dictates how big the play area is.
#[derive(Debug, Clone, Copy)]
pub struct WorldSize(i32, i32);
impl WorldSize {
    /// Creates a new WorldSize.
    pub fn new(size: i32) -> WorldSize {
        WorldSize(-size, size)
    }
    /// Creates a new, randomized WorldSize.
    pub fn randomize(min: i32, max: i32) -> WorldSize {
        let mut rng = rand::thread_rng();
        let max_range = rng.gen_range(min..max);
        WorldSize(-max_range, max_range)
    }
    /// Returns the maximum and minimum x and y values of a World.
    pub fn get_values(&self) -> (i32, i32) {
        (self.0, self.1)
    }
}
