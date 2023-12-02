use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut sum: Vec<u32> = vec![];
        for line in lines.flatten() {
            //let numbers: Vec<char> = line.as_str().chars().filter(|c| c.is_numeric()).collect();
            let numbers = extract_numeric_values(line);
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

fn extract_numeric_values(s: String) -> Vec<char> {
    let mut number_map = HashMap::new();
    number_map.insert("one", '1');
    number_map.insert("two", '2');
    number_map.insert("three", '3');
    number_map.insert("four", '4');
    number_map.insert("five", '5');
    number_map.insert("six", '6');
    number_map.insert("seven", '7');
    number_map.insert("eight", '8');
    number_map.insert("nine", '9');

    let mut current_word = String::from("");
    let mut result: Vec<char> = Vec::new();

    for c in s.chars() {
        if c.is_alphabetic() {
            current_word.push(c);

            if !current_word.is_empty()
                && number_map
                    .keys()
                    .any(|&element| current_word.contains(element))
            {
                let word: String = number_map
                    .keys()
                    .filter(|&&element| current_word.contains(element))
                    .cloned()
                    .collect();

                if let Some(value) = number_map.get(&word.as_str()) {
                    result.push(*value);
                }

                let last_char = current_word.chars().last().unwrap();
                current_word.clear();
                current_word.push(last_char)
            }
        } else {
            result.push(c);
        }
    }

    result
}
