use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_file_lines_iterator(day: &str, input: &str) -> io::Lines<io::BufReader<File>> {
    let input = &format!("./inputs/{}-{}.txt", day, input);
    read_lines(input).unwrap_or_else(|_| panic!("File '{input}' should be accesible"))
}

fn part_one(mut lines: io::Lines<io::BufReader<File>>) -> u32 {
    let mut increment_count: u32 = 0;
    let mut last_value: u32 = lines.next().unwrap().unwrap().parse::<u32>().unwrap();
    for number in lines {
        if let Ok(number) = number {
            let current_value: u32 = number.parse::<u32>().unwrap();
            if current_value > last_value {
                increment_count += 1
            }
            last_value = current_value;
        }
    }
    increment_count
}

fn part_two(mut lines: io::Lines<io::BufReader<File>>) -> u32 {
    let mut idx = 0;
    let mut last_three_values = lines
        .by_ref()
        .take(3)
        .map(|x| x.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let mut increment_count: u32 = 0;
    for number in lines {
        if let Ok(number) = number {
            let current_value: u32 = number.parse::<u32>().unwrap();
            if current_value > last_three_values[idx] {
                increment_count += 1
            }
            last_three_values[idx] = current_value;
            idx = (idx + 1) % 3;
        }
    }
    increment_count
}

pub fn main(day: &str, part: &str, input: &str) -> u32 {
    let lines = get_file_lines_iterator(day, input);
    match part {
        "part1" => part_one(lines),
        "part2" => part_two(lines),
        _ => panic!(""),
    }
}
