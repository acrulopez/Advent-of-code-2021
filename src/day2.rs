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

fn part_one(lines: io::Lines<io::BufReader<File>>) -> u32 {
    let mut h_pos: i32 = 0;
    let mut d_pos: i32 = 0;
    for line in lines {
        if let Ok(line) = line {
            let (direction, step): (&str, i32) = {
                let mut iter = line.split(" ");
                (
                    iter.next().unwrap(),
                    iter.next().unwrap().parse::<i32>().unwrap(),
                )
            };
            match direction {
                "forward" => h_pos += step,
                "down" => d_pos += step,
                "up" => d_pos -= step,
                _ => panic!("Unknown direction: {}", direction),
            }
        }
    }
    return (h_pos * d_pos) as u32;
}

fn part_two(lines: io::Lines<io::BufReader<File>>) -> u32 {
    let mut h_pos: i32 = 0;
    let mut d_pos: i32 = 0;
    let mut aim: i32 = 0;
    for line in lines {
        if let Ok(line) = line {
            let (direction, step): (&str, i32) = {
                let mut iter = line.split(" ");
                (
                    iter.next().unwrap(),
                    iter.next().unwrap().parse::<i32>().unwrap(),
                )
            };
            match direction {
                "forward" => {
                    h_pos += step;
                    d_pos += step * aim;
                }
                "down" => aim += step,
                "up" => aim -= step,
                _ => panic!("Unknown direction: {}", direction),
            }
        }
    }
    return (h_pos * d_pos) as u32;
}

pub fn main(day: &str, part: &str, input: &str) -> u32 {
    let lines = get_file_lines_iterator(day, input);
    match part {
        "part1" => part_one(lines),
        "part2" => part_two(lines),
        _ => panic!(""),
    }
}
