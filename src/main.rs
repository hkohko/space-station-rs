use space_station::prelude::*;
use space_station::Commands::{Empty, Mine, MoveTo, Ping, Recharge, SpaceShipInfo};
use std::cell::RefCell;
use std::io;
fn main() {
    let world = World::new(500, 200, 50, 1, 1, 1, 100);
    let (spc_ship_name, mtr_ship_name) = name_inputs();
    game_loop(mtr_ship_name, spc_ship_name, world)
}
fn game_loop(mtr_ship_name: String, spc_ship_name: String, world: World) {
    let mut mtr_ship = MotherShip::new(spc_ship_name.as_str(), &world);
    let mut spc_ship = SpaceShip::new(mtr_ship_name.as_str(), &world);
    let list_of_commands = vec!["move", "mine", "recharge", "sinfo", "ping"];

    loop {
        println!();
        let cmd = get_command();
        let cmdkind = match validate_commands(cmd, &list_of_commands) {
            Some(cmd) => parse_command(cmd),
            None => Commands::Empty,
        };
        match cmdkind {
            MoveTo => handle_move(&mut spc_ship, &world),
            Mine => handle_mine(&mut spc_ship, &world),
            Recharge => handle_recharge(&mut spc_ship, &mut mtr_ship),
            SpaceShipInfo => handle_sinfo(&spc_ship),
            Ping => handle_ping(&spc_ship),
            Empty => continue,
        }
    }
}
fn handle_move(spc_ship: &mut SpaceShip, world: &World) {
    let mut x = 0;
    let mut y = 0;
    println!("\nCurrent location: {:?}", spc_ship.get_location());
    let cmd = get_input("Move to: ");
    let split_by_comma = cmd.split(',').collect::<Vec<&str>>();
    if split_by_comma.len() != 2 {
        println!("Invalid Coordinate.\nPlease provide an x,y coordinate");
        return;
    }
    for (idx, coords) in split_by_comma.iter().enumerate() {
        let trim = coords.trim();
        match trim.parse::<i32>() {
            Ok(val) => {
                if idx == 0 {
                    x = val
                } else {
                    y = val
                }
            }
            Err(e) => {
                println!("{e}");
                return;
            }
        }
    }
    let to = Coordinates::new(x, y, world.play_area);
    spc_ship.to_location(to);
}
fn handle_mine(spc_ship: &mut SpaceShip, world: &World) {
    let id = get_input("Enter resource ID: ");
    let id_as_i32 = match id.parse::<i32>() {
        Err(e) => {
            println!("{e}");
            return;
        }
        Ok(val) => val,
    };
    let resources = &world.spawned_resources;
    for refcell_rsc in resources.iter() {
        match RefCell::try_borrow_mut(refcell_rsc) {
            Err(e) => println!("{e}"),
            Ok(rsc) => {
                if rsc.get_id() == id_as_i32 {
                    let mut to_mine = rsc;
                    spc_ship.get_env_resources(&mut to_mine);
                }
            }
        }
    }
}
fn handle_ping(spc_ship: &SpaceShip) {
    spc_ship.ping()
}
fn handle_recharge(spc_ship: &mut SpaceShip, mtr_ship: &mut MotherShip) {
    spc_ship.teleport(mtr_ship);
    spc_ship.recharge(mtr_ship);
}
fn handle_sinfo(spc_ship: &SpaceShip) {
    println!();
    spc_ship.display_info();
    println!();
    spc_ship.display_resources();
    println!();
    spc_ship.display_storage();
    println!();
}
fn parse_command(input_cmd: String) -> Commands {
    match input_cmd.as_str() {
        "move" => MoveTo,
        "mine" => Mine,
        "recharge" => Recharge,
        "sinfo" => SpaceShipInfo,
        "ping" => Ping,
        _ => Empty,
    }
}
fn get_command() -> String {
    get_input("Enter Command: ")
}
fn validate_commands(command: String, cmd_list: &Vec<&str>) -> Option<String> {
    if command.is_empty() {
        return None;
    }
    if cmd_list.contains(&command.as_str()) {
        Some(command)
    } else {
        println!(
            "Command not found! available commands:\n\n{}",
            cmd_list.join(", ")
        );
        None
    }
}
fn name_inputs() -> (String, String) {
    let mut spc_ship = String::new();
    let mut mtr_ship = String::new();
    let stdin = io::stdin();
    println!("Name of mother ship:");
    match stdin.read_line(&mut spc_ship) {
        Ok(_) => (),
        Err(e) => println!("{e}"),
    }
    println!("Name of space ship:");
    match stdin.read_line(&mut mtr_ship) {
        Ok(_) => (),
        Err(e) => println!("{e}"),
    }
    (
        spc_ship.trim().to_uppercase(),
        mtr_ship.trim().to_uppercase(),
    )
}
fn get_input(msg: &str) -> String {
    println!("{msg}");
    let mut s = String::with_capacity(50);
    let stdin = io::stdin();
    match stdin.read_line(&mut s) {
        Ok(_) => (),
        Err(e) => println!("{e}"),
    }
    s.trim().to_lowercase()
}
