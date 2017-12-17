extern crate day4;
use std::fs::File;
use std::io::Read;
fn main() {
    let mut pass_file = File::open("./src/passwrds.txt").expect("you numpty");

    let mut passes = String::new();
    pass_file
        .read_to_string(&mut passes)
        .expect("you dun goofed");

    let no_reps = |s: &str| day4::is_pass_non_repeaty(s);
    let no_anas = |s: &str| day4::contains_no_anagrams(s);

    println!("not gold {}", day4::count_valid(&passes, &no_reps));
    println!("gold {}", day4::count_valid(&passes, &no_anas));
}
