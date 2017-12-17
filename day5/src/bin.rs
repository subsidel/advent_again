extern crate day5;
use std::fs::File;
use std::io::Read;
fn main() {
    let mut sergent_hops_file = File::open("./src/big-bunny-hops.txt").expect("you numpty");

    let mut hop_map = String::new();
    sergent_hops_file
        .read_to_string(&mut hop_map)
        .expect("you dun goofed");

    let mut robo_judy_hops = day5::RoboBunny::new(&hop_map);
    println!("not gold {}", robo_judy_hops.escape());

    let mut robo_sergeant_hopper = day5::RoboBunny::new(&hop_map);
    robo_sergeant_hopper.see_a_big_nasty_fox_wolf();
    println!("gold {}", robo_sergeant_hopper.escape());
}
