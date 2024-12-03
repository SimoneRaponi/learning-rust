use std::fs::File;
use std::io::{self, Read};
use regex::Regex;

fn main() -> io::Result<()> {

    // Open the file `puzzle_input.txt` and handle potential errors
    let mut file = File::open("puzzle_input.txt")?;

    // Initialize a string to store the file's contents
    let mut contents = String::new();

    // Read the contents of the file into the string
    file.read_to_string(&mut contents)?;

    // Define a regular expression pattern to match specific function calls:
    // "mul(<num>,<num>)", "do()", and "don't()"
    let pattern = r"mul\(\d+,\d+\)|do\(\)|don't\(\)";

    // Compile the regular expression
    let re = Regex::new(pattern).unwrap();

    let mut res = 0;

    // Flag to track whether multiplication should be performed
    let mut flag = true;

    // Iterate over all regex captures in the input string
    for capture in re.captures_iter(&contents) {
        
        // Extract the matched substring from the capture
        let match_str = capture.get(0).unwrap().as_str();
        
        if match_str == "do()" {
            flag = true; // Enable multiplication flag
        } else if match_str == "don't()" {
            flag = false; // Disable multiplication flag
        } else if match_str.starts_with("mul(") {
            if flag {
                // Extract the numbers from the "mul(<x>,<y>)" format
                let nums = &match_str[4..match_str.len() - 1]; // Remove "mul(" and ")"

                // Split the string into parts (x and y)
                let parts: Vec<&str> = nums.split(',').collect();

                // Parse the individual parts into integers
                let x: i32 = parts[0].parse().unwrap();
                let y: i32 = parts[1].parse().unwrap();
                res += x * y;
            }
        }
    }

    println!("{}", res);

    Ok(())
}
