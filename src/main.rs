use crate::{
    lcp_finding_algorithm::find_lcp,
    parse_input::{print_parsed_input, read_input_file, simplify_parsed_input},
};

mod lcp_finding_algorithm;
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

    println!("-----------------------------------------------------------------------------------------\n\n\n");

    if let Some(found_lcp_map) = find_lcp(
        &active_source,
        &passive_source,
        &active_target,
        &passive_target,
    ) {
        println!("An LCP-map can be constructed from the following map on representatives:");
        for (key, value) in found_lcp_map {
            for label in key {
                print!("{label} ");
            }
            print!("  ->   ");
            for label in value {
                print!("{label} ");
            }
            println!();
        }
    } else {
        println!("No LCP-maps exist from source problem to target.");
    }
}
