use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut sum_of_ids = Vec::<u32>::new();
        for line in lines.flatten() {
            // check possible
            let possible = check_possibility(12, 13, 14, &line);
            // if possible get id
            if possible {
                let id = get_id(&line);
                sum_of_ids.push(id);
            }
        }
        let result = sum_of_ids.iter().sum::<u32>();
        println!("{}", result);
    }
}

fn check_possibility(num_reds: u32, num_green: u32, num_blue: u32, line: &str) -> bool {
    let sets = get_sets(line);
    let mut reds: Vec<u32> = vec![];
    let mut greens: Vec<u32> = vec![];
    let mut blues: Vec<u32> = vec![];
    for set in sets {
        let balls = process_set(set);
        let (red, green, blue) = extract_numbers_from_balls(balls);
        reds.push(red);
        greens.push(green);
        blues.push(blue);
    }

    let max_red = *reds.iter().max().unwrap();
    let max_green = *greens.iter().max().unwrap();
    let max_blue = *blues.iter().max().unwrap();

    max_red <= num_reds && max_green <= num_green && max_blue <= num_blue
}

fn get_sets(line: &str) -> Vec<&str> {
    let sets: Vec<&str> = line
        .split(':')
        .nth(1)
        .unwrap_or("")
        .split(';')
        .map(|game| game.trim())
        .collect();
    sets
}

fn process_set(set: &str) -> Vec<&str> {
    let balls: Vec<&str> = set.split(',').map(|round| round.trim()).collect();
    // println!("{:?}", balls);
    balls
}

fn extract_numbers_from_balls(balls: Vec<&str>) -> (u32, u32, u32) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for ball in balls {
        let parts: Vec<&str> = ball.split_whitespace().collect();
        let number: u32 = parts.first().unwrap().parse().unwrap();
        let color = *parts.last().unwrap();
        match color {
            "red" => red += number,
            "green" => green += number,
            "blue" => blue += number,
            _ => (),
        }
    }
    let numbers_by_colors = (red, green, blue);
    numbers_by_colors
}

fn get_id(line: &str) -> u32 {
    let mut id = String::new();
    for c in line.chars() {
        if c.is_numeric() {
            id.push(c)
        } else if c == ':' {
            break;
        }
    }
    id.parse().unwrap_or(0)
}

fn read_lines<P>(file: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file)?;
    Ok(io::BufReader::new(file).lines())
}
