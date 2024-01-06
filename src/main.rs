use space_station::{SpaceShip, FoodWater, GenericInfo};
fn main() {
    let new_space_station = SpaceShip::new("Zeus");
    println!("{new_space_station:?}");
    new_space_station.display_info();
}
