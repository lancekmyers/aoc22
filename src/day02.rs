use std::fs::File;
use std::io::{BufRead, BufReader};

enum Result {
    Win, Lose, Draw
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum RPS {
    Rock, Paper, Scissors 
}

impl RPS {
    fn beats(self) -> RPS {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }
    fn loses_to(self) -> RPS {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }
}

fn parse_line_a(line : String) -> (RPS, RPS) {
    let opp = match line.chars().nth(0).unwrap() {
        'A' => RPS::Rock,
        'B' => RPS::Paper,
        'C' => RPS::Scissors,
        _ => panic!("Bad game line {}", line)
    };
    let you = match line.chars().nth(2).unwrap() {
        'X' => RPS::Rock,
        'Y' => RPS::Paper,
        'Z' => RPS::Scissors, 
        _ => panic!("Bad game line {}", line)
    };
    (opp, you)
}

fn parse_line_b(line : String) -> (RPS, Result) {
    let opp = match line.chars().nth(0).unwrap() {
        'A' => RPS::Rock,
        'B' => RPS::Paper,
        'C' => RPS::Scissors,
        _ => panic!("Bad game line {}", line)
    };
    let you = match line.chars().nth(2).unwrap() {
        'X' => Result::Lose,
        'Y' => Result::Draw,
        'Z' => Result::Win, 
        _ => panic!("Bad game line {}", line)
    };
    (opp, you)
}

fn need_to_play(opp : RPS, want : Result) -> RPS {
    match want {
        Result::Win => opp.loses_to(),
        Result::Lose => opp.beats(),
        Result::Draw => opp,
    }
}

fn play(o: RPS, y : RPS) -> Result {
    match (o, y) {
        (RPS::Rock, RPS::Rock) => Result::Draw,
        (RPS::Paper, RPS::Paper) => Result::Draw,
        (RPS::Scissors, RPS::Scissors) => Result::Draw,
        (RPS::Rock, RPS::Paper) => Result::Win,
        (RPS::Paper, RPS::Scissors) => Result::Win,
        (RPS::Scissors, RPS::Rock) => Result::Win,
        _ => Result::Lose,
    }
}

fn score(o : RPS, y : RPS) -> i32 {
    let shape_score = match y {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3
    };
    let result_score = match play(o, y) {
        Result::Win  => 6,
        Result::Lose => 0,
        Result::Draw => 3
    };
    shape_score + result_score
}


pub fn soln(path : &str) -> (i32, i32) {
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines().flatten(); //lines(file);

    let mut a : i32 = 0;
    let mut b : i32 = 0;
    for line in lines {
        let (o, y) = parse_line_a(line.clone());
        let (_o, w) = parse_line_b(line);
        let y_ = need_to_play(o, w);
        a += score(o, y);
        b += score(o, y_);
    }

//    let a = lines.map(parse_line).map(|(o, y)| score(o, y)).sum();
    (a, b)
}


#[cfg(test)]
mod tests{
    use crate::day02::*;
    #[test]
    fn parsing() {
        assert_eq!((RPS::Rock, RPS::Rock), parse_line_a("A X".to_string()))
    }
    #[test]
    fn given_test() {
        let (a, b) = soln("data/test02.txt");
        assert_eq!(a, 15);
        assert_eq!(b, 12);
    }
}