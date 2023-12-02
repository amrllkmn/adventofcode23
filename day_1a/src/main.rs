use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut sum: Vec<u32> = vec![];
        for line in lines.flatten() {
            let numbers: Vec<char> = line.as_str().chars().filter(|c| c.is_numeric()).collect();
            let num_str: String = process_chars(numbers).into_iter().collect();
            sum.push(num_str.parse().unwrap_or(0));
        }
        let result = sum.iter().sum::<u32>();
        println!("{}", result);
    }
}

fn read_lines<P>(file: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_chars(chars: Vec<char>) -> Vec<char> {
    if chars.len() > 1 {
        vec![chars[0], *chars.last().unwrap()]
    } else {
        vec![chars[0]; 2]
    }
}
