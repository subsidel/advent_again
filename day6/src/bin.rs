extern crate day6;
use std::fs::File;
use std::io::Read;
fn main() {
    let mut echo_23 = File::open("./src/mems.teckst").expect("you numpty");

    let mut leet_memory_hacks = String::new();
    echo_23
        .read_to_string(&mut leet_memory_hacks)
        .expect("you dun goofed");

    let (not_gold, gold) = day6::find_loop(&leet_memory_hacks);
    println!("not gold {}", not_gold);
    println!("gold {}", gold);
}
