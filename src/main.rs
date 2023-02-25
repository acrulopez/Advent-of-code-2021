mod day1;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 4
        || (args[2] != "example" && args[2] != "input" || args[3] != "part1" && args[3] != "part2")
    {
        panic!(
            "Usage 'cargo run day{{X}} {{inputFile}} {{part}}'. Input 
        file should be either 'example' or 'input'. Part should be 'part1' or 'part2'"
        )
    };

    let result = day1::main(&args[2], &args[3]);
    println!("Result: {}", result);
}
