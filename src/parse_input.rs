use std::fs;

type ParseResult<T> = Result<T, String>;

pub(super) fn read_input_file(file_name: &str) -> ParseResult<Vec<Vec<String>>> {
    fs::read_to_string(["input/", file_name].concat())
        .map_err(|e| format!("Reading input {} failed: {e}", file_name))
        .and_then(|input_string| {
            parse_input(&input_string)
                .map_err(|msg| format!("Parsing input {} failed {msg}", file_name))
        })
}

pub(super) fn simplify_parsed_input(mut parsed_input: Vec<Vec<String>>) -> Vec<Vec<String>> {
    parsed_input.iter_mut().for_each(|row| row.sort_unstable());
    parsed_input.sort_unstable();
    parsed_input.dedup();
    parsed_input
}

pub(super) fn print_parsed_input(parsed_input: &Vec<Vec<String>>) {
    for configuration in parsed_input {
        for label in configuration {
            print!("{label} ");
        }
        println!();
    }
    println!();
}

// Parses a set of configurations
fn parse_input(input: &str) -> ParseResult<Vec<Vec<String>>> {
    let mut rows = input
        .trim()
        .split('\n')
        .map(|row| {
            row.trim()
                .split(' ')
                .filter(|shorthand_expression| !shorthand_expression.is_empty())
                .map(String::from)
                .collect::<Vec<String>>()
        })
        .filter(|row| !row.is_empty())
        .collect::<Vec<_>>();

    let row_length = rows
        .iter()
        .map(Vec::len)
        .map(Ok)
        .reduce(|acc, row_length| {
            acc.and_then(|acc_value| {
                row_length.and_then(|row_length_value| {
                    if row_length_value == acc_value {
                        Ok(acc_value)
                    } else {
                        Err(format!(
                            "Mismatched row lengths: expected {}, found {}.",
                            acc_value, row_length_value
                        ))
                    }
                })
            })
        })
        .ok_or("No nonempty rows.".to_string())??;

    for i in 0..row_length {
        rows = ParseResult::<Vec<Vec<Vec<String>>>>::from_iter(
            rows.iter().map(|row| expand_at_index(row, i)),
        )?
        .concat()
    }
    Ok(rows)
}

fn unravel_shorthand(shorthand_expression: &[char]) -> ParseResult<Vec<String>> {
    let mut expanded = Vec::new();
    let mut i = 0;
    while i < shorthand_expression.len() {
        match shorthand_expression[i] {
            '(' => {
                // Read the string between brackets as one label
                let mut label = String::new();
                loop {
                    i += 1;
                    if i < shorthand_expression.len() {
                        match shorthand_expression[i] {
                            ')' => break,
                            '(' => return Err("Unexpected '('.".to_string()),
                            character => {
                                if character.is_whitespace() {
                                    return Err(format!("Unexpected whitespace {}.", character));
                                } else {
                                    label.push(character)
                                }
                            }
                        }
                    } else {
                        return Err("Missing ')'.".to_string());
                    }
                }
                if label.is_empty() {
                    return Err("Found an empty label ().".to_string());
                } else {
                    expanded.push(label)
                }
            }
            ')' => {
                return Err("Unexpected ')'.".to_string());
            }
            label => {
                if label.is_whitespace() {
                    return Err(format!("Unexpected whitespace {}.", label));
                } else {
                    expanded.push(label.to_string())
                }
            }
        }
        i += 1;
    }
    Ok(expanded)
}

// Unravels shorthand at index and expands to different rows corresponding to the different available labels
fn expand_at_index(row: &[String], index: usize) -> ParseResult<Vec<Vec<String>>> {
    row.get(index)
        .ok_or("Index somehow out of range.".to_string())
        .and_then(|shorthand_expression| {
            unravel_shorthand(&shorthand_expression.chars().collect::<Vec<char>>())
        })
        .map(|unraveled_labels| {
            unraveled_labels
                .iter()
                .map(|unraveled_label| {
                    let mut row_with_label = Vec::from(row);
                    row_with_label[index] = unraveled_label.clone();
                    row_with_label
                })
                .collect()
        })
}
