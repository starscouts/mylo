mod stdio;

use std::fs;

fn main() {
    let contents = fs::read_to_string("./test.ml")
        .expect("Should have been able to read the file");

    for line in contents.lines() {
        let mut in_double_quotes = false;
        let mut in_single_quotes = false;
        let mut escaping = false;

        let input = line.trim();
        let input_split = input.split(|char| match char {
            '"' if !in_single_quotes && !escaping => {
                in_double_quotes = !in_double_quotes;
                false
            }
            '\'' if !in_double_quotes && !escaping => {
                in_single_quotes = !in_single_quotes;
                false
            }
            '\\' if !escaping => {
                escaping = true;
                false
            },
            ' ' => !in_double_quotes && !in_single_quotes,
            _ => {
                escaping = false;
                false
            },
        });

        let mut parsed = input_split.map(|split| {
            match split.chars().next().unwrap() {
                '\'' | '"' => {
                    let end = split.len() - 1;
                    split[1..end].replace("\\n", "\n").replace("\\t", "\t").replace("\\r", "\r").replace('\\', "")
                }
                _ => split.replace("\\n", "\n").replace("\\t", "\t").replace("\\r", "\r").replace('\\', "")
            }
        });
        let args: Vec<String> = parsed.collect();

        if args.len() == 0 || args[0].starts_with(":") || args[0].to_lowercase() == "rem" {
            continue;
        }

        match args[0].as_str() {
            "print" => {
                stdio::print(args[1..].to_vec());
            },
            _ => {
                panic!("Unrecognized keyword: {}", args[0]);
            }
        }
    }
}
