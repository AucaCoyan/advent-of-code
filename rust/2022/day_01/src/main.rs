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

    let printable_string: Vec<&str> = file.split("\r\n\r\n").collect();
    // print the entire load of the elf
    // println!("This is the output: {:?}", printable_string);
    for elf in printable_string {
        println!("This is one elf {:?}", elf);
        // old way to do it:
        // it returns a `Split<&str>` which prints almost jargon to the console
        // let all_loads = elf.split("\r\n");
        // if you split an object, it returns Split structure that have a lot of methods
        // it doesnt return a vector with all the elements joined.
        // You have to explicity tell rust that you want them collected

        let all_loads: Vec<&str> = elf.split("\r\n").collect();
        println!("This is the output: {:?}", all_loads);
        let mut total_elf_load = 0;

        let this_elf_load: f32 = all_loads.iter().sum();

        println!("This elf has a total load of {}", total_elf_load);

        break;
    }
}
