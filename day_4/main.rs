use clap::clap_app;
use regex::Regex;
use std::fs;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Board {
    numbers: Vec<Vec<u32>>,
    chosen: Vec<Vec<bool>>
}

impl Board {
    fn choose(&mut self, number: u32) {
        for (row_index, row) in self.numbers.iter().enumerate() {
            for (column_index, &value) in row.iter().enumerate() {
                self.chosen[row_index][column_index] |= value == number;
            }
        }
    }
    fn is_winning(&self) -> bool {
        fn has_full_row(chosen: &Vec<Vec<bool>>) -> bool {
            chosen.iter().any(|row: &Vec<bool>| {
                row.iter().all(|&x| x)
            })
        }
        has_full_row(&self.chosen) || has_full_row(&transpose(&self.chosen))
    }
    fn sum_of_not_chosen_numbers(&self) -> u32 {
        self.numbers.iter().zip(self.chosen.iter()).map(|(row_number, row_chosen)| {
            row_number.iter().zip(row_chosen).map(|(&number, &chosen)| if chosen { 0u32 } else { number }).sum::<u32>()
        }).sum()
    }
}

// https://stackoverflow.com/a/64499219/7396293
fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> where T: Clone, {
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| {
            v.iter().map(|inner| {
                inner[i].clone()
            }).collect::<Vec<T>>()
        }).collect()
}

type BoardMatrix = Vec<Vec<u32>>;

fn parse_number(chosen_number: &str) -> u32 {
    chosen_number
        .parse::<u32>()
        .expect("Not able to parse number as u32 ")
}

fn parse_boards(inputs: Vec<&str>) -> Vec<Board> {
    let parse_row = |row: &str| {
        row.split_whitespace().map(parse_number).collect::<Vec<u32>>()
    };
    let parse_board = |board_string: &&str| {
        let board: BoardMatrix = board_string
            .lines()
            .map(parse_row)
            .collect();

        let number_of_rows = board.len();
        let number_of_columns = board.first().unwrap().len();

        Board {
            numbers: board,
            chosen: vec![vec![false; number_of_rows]; number_of_columns]
        }
    };
    inputs.iter()
        .map(parse_board).collect()
}

// TODO: config rustfmt
fn main() {
    let matches = clap_app!(("aoc2021") =>
        (version: "0.1.0")
        (author: "Benjamin Potzmann <benjamin.potzmann@gmail.com>")
        (about: "Advent Of Code 2021 Day 4")
        (@arg INPUT: +required "The path to the input file.")
    )
    .get_matches();

    let input_path = matches.value_of("INPUT")
        .expect("Parameter INPUT is missing");
    println!("Path: {}", input_path);

    let contents_raw = fs::read_to_string(input_path)
        .expect("Something went wrong reading the file");
    let contents = contents_raw.trim();

    let (chosen_numbers, boards) = parse_input(contents);

    println!("Task1: {}", task1(&chosen_numbers, boards.clone()));
    println!("Task2: {}", task2(&chosen_numbers, boards));
}

fn parse_input(contents: &str) -> (Vec<u32>, Vec<Board>) {
    let empty_line_seperator = Regex::new(r"(\r?\n){2,}")
        .expect("Invalid regex");
    let input_parts: Vec<&str> = empty_line_seperator
        .split(contents).collect();

    let chosen_numbers: Vec<u32> = input_parts[0]
        .split(",")
        .map(parse_number).collect();

    let boards = parse_boards(input_parts[1..].to_vec());

    (chosen_numbers, boards)
}

fn task1(chosen_numbers: &Vec<u32>, mut boards: Vec<Board>) -> u32 {
    for number in chosen_numbers {
        for board in boards.iter_mut() {
            board.choose(*number);
            if board.is_winning() {
                return number * board.sum_of_not_chosen_numbers();
            }
        }
    }
    panic!("Fucked up");
}

fn task2(chosen_numbers: &Vec<u32>, mut boards: Vec<Board>) -> u32 {
    let mut last_winning_score: Option<u32> = None;
    let mut winning_board_indices = HashSet::new();

    for number in chosen_numbers {
        for (index, board) in boards.iter_mut().enumerate() {
            if !winning_board_indices.contains(&index) {
                board.choose(*number);
                if board.is_winning() {
                    let score = number * board.sum_of_not_chosen_numbers();
                    last_winning_score = Some(score);
                    winning_board_indices.insert(index);
                }
            }
        }
    }
    last_winning_score.expect("Fucked up")
}

