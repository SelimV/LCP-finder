use crate::parse_input::{print_parsed_input, read_input_file, simplify_parsed_input};

mod parse_input;

fn main() {
    println!("Parse input:");
    let active_source = simplify_parsed_input(read_input_file("active_source").unwrap());
    let passive_source = simplify_parsed_input(read_input_file("passive_source").unwrap());
    let active_target = simplify_parsed_input(read_input_file("active_target").unwrap());
    let passive_target = simplify_parsed_input(read_input_file("passive_target").unwrap());

    println!("active_source");
    print_parsed_input(&active_source);
    println!("passive_source");
    print_parsed_input(&passive_source);
    println!("active_target");
    print_parsed_input(&active_target);
    println!("passive_target");
    print_parsed_input(&passive_target);
}
