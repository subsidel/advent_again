extern crate day2;
use std::fs::File;
use std::io::Read;
fn main() {
    let mut file_thing =
        File::open("./src/input.clearly_a_spreadsheet_dont_lie").expect("you numpty");

    let mut a_whopper_of_a_string = String::new();
    file_thing
        .read_to_string(&mut a_whopper_of_a_string)
        .expect("you dun goofed");

    println!("not gold {}", day2::do_the_thing(&a_whopper_of_a_string));
    println!("gold {}", day2::do_the_thing_gold(&a_whopper_of_a_string));
}
