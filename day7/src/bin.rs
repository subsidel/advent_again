extern crate day7;
use std::fs::File;
use std::io::Read;
fn main() {
    let mut fyul = File::open("./src/notes.notes").expect("you numpty");

    let mut best_top_ever = String::new();
    fyul.read_to_string(&mut best_top_ever)
        .expect("you dun goofed");

    println!(
        "not gold {:?}",
        day7::find_tree_base(day7::parse_notes(&best_top_ever))
    );


    let lines = day7::parse_notes(&best_top_ever);
    let base = day7::find_tree_base(lines.clone());
    let tree = day7::form_tree(&base.unwrap(), lines);
    println!("{:#?}", tree);

    // ceebs with part 2, look how pretty that tree is though
}
