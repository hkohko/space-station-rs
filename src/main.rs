use space_station::prelude::*;
use space_station::Commands::{MoveTo, Mine, Recharge, SpaceShipInfo, Empty};
use std::io;
fn main() {
    let world = World::new(
        500,
        200,
        50,
        1,
        1,
        1,
        100
    );
    let (spc_ship_name, mtr_ship_name) = name_inputs();
    game_loop(mtr_ship_name, spc_ship_name, world)
}
fn game_loop(mtr_ship_name: String, spc_ship_name: String, world: World) {
    let mut mtr_ship = MotherShip::new(spc_ship_name.as_str(), &world);
    let mut spc_ship = SpaceShip::new(mtr_ship_name.as_str(), &world);
    let list_of_commands = vec!["move", "mine", "recharge", "sinfo"];
    loop {
        let cmd = get_command();
        let cmdkind = match validate_commands(cmd, &list_of_commands) {
            Some(cmd) => parse_command(cmd),
            None => Commands::Empty,
        };
        match cmdkind {
            MoveTo => handle_move(&mut spc_ship, &world),
            Mine => handle_mine(),
            Recharge => handle_recharge(),
            SpaceShipInfo => handle_sinfo(),
            Empty => (),
        }
    }
}
fn handle_move(spc_ship: &mut SpaceShip, world: &World) {
    let mut x = 0;
    let mut y = 0;
    println!("\nCurrent location: {:?}", spc_ship.get_location());
    let cmd = get_input("Move to: ");
    let split_by_comma = cmd.split(",").collect::<Vec<&str>>();
    for (idx, coords) in split_by_comma.iter().enumerate() {
        let trim = coords.trim();
        match trim.parse::<i32>() {
            Ok(val) => {
                if idx == 0 {
                    x = val
                } else {
                    y = val
                }
            },
            Err(e) => println!("{e}"),
        }
    }
    
    let to = Coordinates::new(x, y, world.play_area);
    spc_ship.to_location(to);
}
fn handle_mine() {
    
}
fn handle_recharge() {
    
}
fn handle_sinfo() {
    
}
fn parse_command(input_cmd: String) -> Commands {
    match input_cmd.as_str() {
        "move" => MoveTo,
        "mine" => Mine,
        "recharge" => Recharge,
        "sinfo" => SpaceShipInfo,
        _ => Empty
    }
}
fn get_command() -> String {
    let cmd = get_input("Enter Command: ");
    cmd

}
fn validate_commands(command: String, cmd_list: &Vec<&str>) -> Option<String>{
    if command.len() == 0 {
        return None
    }
    if cmd_list.contains(&command.as_str()) {
        Some(command)
    } else {
        println!("Command not found! available commands:\n\n {}", cmd_list.join(""));
        None
    }
}
fn name_inputs() -> (String, String){
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
    (spc_ship.trim().to_uppercase(), mtr_ship.trim().to_uppercase())
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