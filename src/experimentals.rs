use crate::prelude::*;
pub fn experimental_main() {
    // let _ = resource_spawning_consuming_idea();
    build_world();
}
fn build_world() {
    let w = World::new(100, 10, 100, 1, 1, 1, 100);
    dbg!(&w);
}
fn resource_spawning_consuming_idea() {
    let new_world = World::randomize(WorldSize::new(1000));
    let mut new_ship = SpaceShip::new("Zeus", &new_world);
    // create a new list with x amount of items.
    let mut v_env_rsc = Vec::with_capacity(10);
    // spawn randomized resources with a for loop and put them in a vector
    for a in 0..10 {
        let new_env = EnvResource::randomize(50, a, WorldSize::new(100));
        v_env_rsc.push(new_env);
    }
    dbg!(&v_env_rsc);
    dbg!(&new_ship);

    for env in v_env_rsc.clone().into_iter() {
        // iterate through clone of the vec, and consume it,
        // while making each product of the for loop mutable.
        let mut new_v = env;
        // take a mutable reference of the item and take it's value
        new_ship.get_env_resources(&mut new_v);
        // replace the item in the original vec `v_env_rsc` with the cloned item
        // that has updated value.
        let indexer = match usize::try_from(new_v.get_id()) {
            Ok(val) => val,
            Err(e) => {
                println!("{e}");
                0usize
            }
        };
        v_env_rsc[indexer] = new_v;
    }
    dbg!(&v_env_rsc);
    dbg!(new_ship);
}
