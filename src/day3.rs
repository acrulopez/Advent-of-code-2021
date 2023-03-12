use std::collections::HashSet;
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
    let mut bit_count: Vec<(i32, i32)> = Vec::new();

    // Read first line to initialize vector
    let first_line = lines.next().unwrap().unwrap();
    for char in first_line.chars() {
        match char {
            '0' => bit_count.push((1, 0)),
            '1' => bit_count.push((0, 1)),
            _ => panic!("Unknown bit: {}", char),
        }
    }

    // Read the rest of the lines
    for line in lines {
        if let Ok(line) = line {
            for (i, char) in line.chars().enumerate() {
                match char {
                    '0' => bit_count[i].0 += 1,
                    '1' => bit_count[i].1 += 1,
                    _ => panic!("Unknown bit: {}", char),
                }
            }
        }
    }

    let base: u32 = 2;
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for (i, bit) in bit_count.iter().rev().enumerate() {
        if bit.0 > bit.1 {
            gamma += base.pow(i as u32);
        } else {
            epsilon += base.pow(i as u32);
        }
    }

    return gamma * epsilon;
}

fn part_two(lines: io::Lines<io::BufReader<File>>) -> u32 {
    let mut zero_set: HashSet<Vec<char>> = HashSet::new();
    let mut one_set: HashSet<Vec<char>> = HashSet::new();

    // Check the most common first bit
    for line in lines {
        if let Ok(line) = line {
            let chars = line.chars().collect();
            if line.chars().next().unwrap() == '0' {
                zero_set.insert(chars);
            } else {
                one_set.insert(chars);
            }
        }
    }

    // Create the initial set for oxygen and CO2
    let mut oxygen_set;
    let mut co2_set;
    if zero_set.len() > one_set.len() {
        oxygen_set = zero_set;
        co2_set = one_set;
    } else {
        oxygen_set = one_set;
        co2_set = zero_set;
    }

    // Get the oxygen bits
    let mut it = 1;
    while oxygen_set.len() > 1 {
        let one_count = oxygen_set
            .iter()
            .fold(0, |sum, chars| if chars[it] == '1' { sum + 1 } else { sum });
        if one_count * 2 >= oxygen_set.len() {
            oxygen_set.retain(|chars| chars[it] == '1');
        } else {
            oxygen_set.retain(|chars| chars[it] == '0');
        }
        it += 1;
    }
    let oxygen_bits = oxygen_set.iter().next().unwrap();

    // Get the CO2 bits
    it = 1;
    while co2_set.len() > 1 {
        let zero_count = co2_set
            .iter()
            .fold(0, |sum, chars| if chars[it] == '0' { sum + 1 } else { sum });

        if zero_count * 2 <= co2_set.len() {
            co2_set.retain(|chars| chars[it] == '0');
        } else {
            co2_set.retain(|chars| chars[it] == '1');
        }
        it += 1;
    }
    let co2_bits = co2_set.iter().next().unwrap();

    let base: u32 = 2;
    let mut oxygen_decimal = 0;
    for (i, bit) in oxygen_bits.iter().rev().enumerate() {
        if bit == &'1' {
            oxygen_decimal += base.pow(i as u32);
        }
    }

    let mut co2_decimal = 0;
    for (i, bit) in co2_bits.iter().rev().enumerate() {
        if bit == &'1' {
            co2_decimal += base.pow(i as u32);
        }
    }

    return (oxygen_decimal * co2_decimal) as u32;
}

pub fn main(day: &str, part: &str, input: &str) -> u32 {
    let lines = get_file_lines_iterator(day, input);
    match part {
        "part1" => part_one(lines),
        "part2" => part_two(lines),
        _ => panic!(""),
    }
}
