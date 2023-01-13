use std::io::{prelude::*, BufReader};
use std::fs::File;

const LOSS: i32 = 0;
const DRAW: i32 = 3;
const WIN: i32 = 6;

const ROCK_THEM: char = 'A';
const PAPER_THEM: char = 'B';
const SCISSORS_THEM: char = 'C';

const ROCK_ME: char = 'X';
const PAPER_ME: char = 'Y';
const SCISSORS_ME: char = 'Z';

const LOSS_2: char = 'X';
const DRAW_2: char = 'Y';
const WIN_2: char = 'Z';

fn points_for_hand(c: char) -> i32 {
    match c {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Invalid input")
    }
}

fn points_for_outcome(c: char) -> i32 {
    match c {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => panic!("Invalid input")
    }
}

fn main() -> std::io::Result<()> {
    let f = File::open("input")?;
    let mut bufreader = BufReader::new(f);

    let mut acc_1 = 0;
    let mut acc_2 = 0;

    loop {
        let mut line = String::new();

        if bufreader.read_line(&mut line)? == 0 {
            break
        }

        let clean = line.trim();
        let them = clean.chars().nth(0).unwrap();
        let me_1 = clean.chars().nth(2).unwrap();
        let outcome_2 = clean.chars().nth(2).unwrap();

        let outcome_1 = match (them, me_1) {
            (ROCK_THEM, ROCK_ME) => DRAW,
            (ROCK_THEM, PAPER_ME) => WIN,
            (ROCK_THEM, SCISSORS_ME) => LOSS,
            (PAPER_THEM, ROCK_ME) => LOSS,
            (PAPER_THEM, PAPER_ME) => DRAW,
            (PAPER_THEM, SCISSORS_ME) => WIN,
            (SCISSORS_THEM, ROCK_ME) => WIN,
            (SCISSORS_THEM, PAPER_ME) => LOSS,
            (SCISSORS_THEM, SCISSORS_ME) => DRAW,
            _ => panic!("Invalid entry")
        };

        acc_1 += points_for_hand(me_1) + outcome_1;

        let me_2 = match (them, outcome_2) {
            (ROCK_THEM, LOSS_2) => SCISSORS_ME,
            (ROCK_THEM, DRAW_2) => ROCK_ME,
            (ROCK_THEM, WIN_2) => PAPER_ME,
            (PAPER_THEM, LOSS_2) => ROCK_ME,
            (PAPER_THEM, DRAW_2) => PAPER_ME,
            (PAPER_THEM, WIN_2) => SCISSORS_ME,
            (SCISSORS_THEM, LOSS_2) => PAPER_ME,
            (SCISSORS_THEM, DRAW_2) => SCISSORS_ME,
            (SCISSORS_THEM, WIN_2) => ROCK_ME,
            _ => panic!("Invalid entry")
        };

        acc_2 += points_for_hand(me_2) + points_for_outcome(outcome_2);
    }

    println!("{}", acc_1);
    println!("{}", acc_2);

    Ok(())
}
