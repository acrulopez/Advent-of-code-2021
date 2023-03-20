use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

// const MATRIX_SIZE: usize = 5; TODO

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

fn compute_sum_non_marked(bingo_table: &Vec<Vec<usize>>, marked_numbers: &Vec<bool>) -> usize {
    let mut sum = 0;
    for row in bingo_table {
        for number in row {
            if !marked_numbers[*number] {
                sum += number;
            }
        }
    }
    return sum;
}

fn part_one(mut lines: io::Lines<io::BufReader<File>>) -> u32 {
    // Represents the (table_number, x ,y) -> is_occupied
    // The index of the vector correspond with the number
    let mut bingo: Vec<Vec<Vec<usize>>> = vec![];
    let mut number_to_cells: Vec<Vec<(usize, usize, usize)>> = vec![vec![]; 100];
    let mut marked_numbers: Vec<bool> = vec![false; 100];

    // Get the bingo numbers
    let bingo_numbers: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    lines.next(); // Remove the first empty line
    bingo.push(vec![]); // Add a new table

    // Parse the bingo tables
    let mut table_number = 0;
    let mut column_number = 0;
    for line in lines {
        if let Ok(line) = line {
            if line.is_empty() {
                bingo.push(vec![]); // Add a new table
                table_number += 1;
                column_number = 0;
                continue;
            }
            let numbers = line
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            for (row_number, number) in numbers.iter().enumerate() {
                number_to_cells[*number].push((table_number, column_number, row_number));
            }
            column_number += 1;
            bingo[table_number].push(numbers);
        }
    }

    for number in bingo_numbers {
        for (table_number, x, y) in &number_to_cells[number] {
            marked_numbers[number] = true;

            // Check column is marked
            let column_finished = bingo[*table_number][*x]
                .iter()
                .all(|&value| marked_numbers[value]);

            if column_finished {
                return (compute_sum_non_marked(&bingo[*table_number], &marked_numbers) * number)
                    as u32;
            }

            let row_finished = bingo[*table_number]
                .iter()
                .map(|row| row[*y])
                .all(|value| marked_numbers[value]);

            if row_finished {
                return (compute_sum_non_marked(&bingo[*table_number], &marked_numbers) * number)
                    as u32;
            }
        }
    }

    return 0;
}

fn part_two(mut lines: io::Lines<io::BufReader<File>>) -> u32 {
    // Represents the (table_number, x ,y) -> is_occupied
    // The index of the vector correspond with the number
    let mut bingo: Vec<Vec<Vec<usize>>> = vec![];
    let mut number_to_cells: Vec<Vec<(usize, usize, usize)>> = vec![vec![]; 100];
    let mut marked_numbers: Vec<bool> = vec![false; 100];
    let mut marked_tables: Vec<bool> = vec![];
    let mut marked_tables_number = 0;

    // Get the bingo numbers
    let bingo_numbers: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    lines.next(); // Remove the first empty line
    bingo.push(vec![]); // Add a new table

    // Parse the bingo tables
    let mut table_number = 0;
    let mut column_number = 0;
    for line in lines {
        if let Ok(line) = line {
            if line.is_empty() {
                bingo.push(vec![]); // Add a new table
                table_number += 1;
                column_number = 0;
                continue;
            }
            let numbers = line
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            for (row_number, number) in numbers.iter().enumerate() {
                number_to_cells[*number].push((table_number, column_number, row_number));
            }
            column_number += 1;
            bingo[table_number].push(numbers);
        }
    }
    marked_tables.extend(std::iter::repeat(false).take(table_number + 1));

    for number in bingo_numbers {
        for (table_number, x, y) in &number_to_cells[number] {
            if marked_tables[*table_number] {
                continue;
            }
            marked_numbers[number] = true;

            // Check all column is marked
            let column_finished = bingo[*table_number][*x]
                .iter()
                .all(|&value| marked_numbers[value]);

            // Check all row is marked
            let row_finished = bingo[*table_number]
                .iter()
                .map(|row| row[*y])
                .all(|value| marked_numbers[value]);

            if column_finished || row_finished {
                marked_tables[*table_number] = true;
                marked_tables_number += 1;
                if marked_tables_number == bingo.len() {
                    return (compute_sum_non_marked(&bingo[*table_number], &marked_numbers) * number)
                        as u32;
                }
            }
        }
    }

    return 1010101010;
}

pub fn main(day: &str, part: &str, input: &str) -> u32 {
    let lines = get_file_lines_iterator(day, input);
    match part {
        "part1" => part_one(lines),
        "part2" => part_two(lines),
        _ => panic!(""),
    }
}
