use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use itertools::Itertools;


// Generate all possible sequences of operators
fn generate_operator_sequences(length: usize) -> Vec<Vec<char>> {
    let operators = vec!['+', '*', '|'];
    if length == 0 {
        return vec![];
    }
    (0..length)
        .map(|_| operators.clone())
        .multi_cartesian_product()
        .collect()
}

/// Reads lines from a file and returns an iterator over them.
/// If the file cannot be opened, an error is returned.
///
/// # Arguments
/// * `filename` - Path to the file to be read.
///
/// # Returns
/// * An iterator over the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calculate_matching_expressions(rows: Vec<Vec<i64>>) -> i64 {

    let mut valid_expressions_total: i64 = 0;
    let mut hash_map:HashMap<usize, Vec<Vec<char>>> = HashMap::new();

    for row in rows.iter() {
        
        // The first value in the sequence represents the target total
        let target_total = row[0];
        // The remaining values are the numbers to operate on
        let numbers = &row[1..];
        // Store the number of operators to produce
        let operators_length = numbers.len()-1;

        let operator_sequences: Vec<Vec<char>>;

        // Get the sequence of operators if already present in the hashMap
        if hash_map.contains_key(&operators_length) {
            operator_sequences = hash_map.get(&operators_length).unwrap().to_vec();
        } else {
            // Generate all possible operator sequences ((len(n)-1)-long)
            operator_sequences = generate_operator_sequences(operators_length);
            hash_map.insert(operators_length, operator_sequences.clone());
        }

        for operators in operator_sequences.iter() {
            let (is_match, matching_value) = evaluate_expression(target_total, numbers,  operators);
            if is_match {
                valid_expressions_total += matching_value;
                // Stop further checks for this row once a match is found
                break;
            }
        }
    }

    valid_expressions_total
}

fn concat_integers(a: i64, b: i64) -> u64 {
    a as u64 * 10u64.pow(b.ilog10() + 1) + b as u64
}

// Evaluate the expression formed by the combination of numbers and operators.
// Returns a tuple: (whether the result matches the target total, value if it matches)
fn evaluate_expression(target_total: i64, numbers: &[i64], operators: &[char]) -> (bool, i64) {
    let mut result = numbers[0];
    for (op, &num) in operators.iter().zip(&numbers[1..]) {
        match op {
            '+' => result += num,
            '*' => result *= num,
            '|' => result = concat_integers(result, num) as i64,
            _ => panic!("Unsupported operator"),
        }
    }

    if result == target_total {
        return (true, target_total);
    }

    (false, 0)
}

fn main() {
    let mut parsed_rows: Vec<Vec<i64>> = Vec::new();

    // Read and parse the input file
    if let Ok(lines) = read_lines("./puzzle_input.txt") {
        for line in lines.flatten() {
            let parts: Vec<&str> = line.split(':').collect();
            let target_total = parts[0].trim().parse::<i64>().unwrap();
            let numbers: Vec<i64> = parts[1]
                .split_whitespace()
                .filter_map(|s| s.trim().parse::<i64>().ok())
                .collect();

            let mut row: Vec<i64> = vec![target_total];
            row.extend(numbers);

            parsed_rows.push(row);
        }
    }

    // Calculate the number of matching expressions
    let result = calculate_matching_expressions(parsed_rows);

    // Output the result
    println!("{}", result);
}
