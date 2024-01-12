use crate::prelude::*;
pub fn experimental_main() {
    let _ = resource_spawning_consuming_idea();
}

fn resource_spawning_consuming_idea() {
    let mut new_ship = SpaceShip::new("Zeus");
    let mut v_env_rsc = Vec::with_capacity(10);
    for a in 0..10 {
        let new_env = EnvResource::randomize(50, a);
        v_env_rsc.push(new_env);
    }
    dbg!(&v_env_rsc);
    dbg!(&new_ship);
    for env in v_env_rsc.clone().into_iter() {
        let mut new_v = env;
        new_ship.get_env_resources(&mut new_v);
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