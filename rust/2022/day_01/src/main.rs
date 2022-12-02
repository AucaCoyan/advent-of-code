// 2022 - day 01
// https://adventofcode.com/2022/day/1

use std::fs;

fn main() {
    /*
    let filenames = fs::read_dir("./src/puzzle.input.txt").expect("oh no, we can't read the file");

    for item_found in filenames {
        println!("Name: {}", item_found.unwrap().path().display())
    }
    */

    let file: String = fs::read_to_string("./src/puzzle-input.txt").expect("Unable to read file");

    let printable_string = file.split("\r\n\r\n");
    for item in printable_string {
        // let number = &item.split_whitespace().as_str();
        println!("This is the output: {:?}", item);
        break;
    }
}
