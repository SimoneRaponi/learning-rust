use std::fs::File;
use std::io::{self, Read};
use regex::Regex;

fn main() -> io::Result<()> {
    let mut file = File::open("puzzle_input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let pattern = r"mul\(\d+,\d+\)|do\(\)|don't\(\)";
    let re = Regex::new(pattern).unwrap();

    let mut res = 0;
    let mut flag = true;

    for capture in re.captures_iter(&contents) {
        let match_str = capture.get(0).unwrap().as_str();
        
        if match_str == "do()" {
            flag = true;
        } else if match_str == "don't()" {
            flag = false;
        } else if match_str.starts_with("mul(") {
            if flag {
                let nums = &match_str[4..match_str.len() - 1]; // Remove "mul(" and ")"
                let parts: Vec<&str> = nums.split(',').collect();
                let x: i32 = parts[0].parse().unwrap();
                let y: i32 = parts[1].parse().unwrap();
                res += x * y;
            }
        }
    }

    println!("{}", res);

    Ok(())
}
