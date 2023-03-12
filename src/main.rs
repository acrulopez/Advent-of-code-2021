mod day1;
mod day2;
mod day3;
use std::time::Instant;

fn main() {
    let mut args = std::env::args();
    args.next();
    let (day, part, input) = (
        args.next().unwrap(),
        args.next().unwrap(),
        args.next().unwrap(),
    );

    if !day.starts_with("day")
        || part != "part1" && part != "part2"
        || input != "example" && input != "input"
    {
        panic!(
            "Usage 'cargo run day{{X}} {{part}} {{inputFile}} '. \n 
            Day: {day} \n
            Part: {part} \n
            Input: {input} \n"
        );
    }

    let start = Instant::now();

    let result = match day.as_str() {
        "day1" => day1::main(&day, &part, &input),
        "day2" => day2::main(&day, &part, &input),
        "day3" => day3::main(&day, &part, &input),
        _ => panic!(""),
    };
    let end = Instant::now();

    println!("Result: {}", result);
    println!("Time taken: {:?}", end - start);
}
