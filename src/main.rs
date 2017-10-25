extern crate service_utils;

use service_utils::islands::{Territory};


fn main() {
    let mut territory = Territory::new();
    println!("{}", territory.add_land(0, 0));
    println!("{}", territory.add_land(1, 0));
    println!("{}", territory.add_land(3, 0));
    println!("{}", territory.add_land(2, 0));
    println!("{}", territory.add_land(4, 4));
    println!("{}", territory.add_land(3, 4));
    println!("{}", territory.add_land(1, 4));
}
