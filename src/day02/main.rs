use std::{fs::File, io::Read};

#[derive(PartialEq, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn for_self(string: &str) -> Option<Move> {
        match string {
            "X" => Some(Move::Rock),
            "Y" => Some(Move::Paper),
            "Z" => Some(Move::Scissors),
            _ => None,
        }
    }
    fn for_opponent(string: &str) -> Option<Move> {
        match string {
            "A" => Some(Move::Rock),
            "B" => Some(Move::Paper),
            "C" => Some(Move::Scissors),
            _ => None,
        }
    }

    fn value(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn value(&self) -> i32 {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }

    fn from_game(player: &Move, opponent: &Move) -> Outcome {
        match (player, opponent) {
            (Move::Paper, Move::Rock) => Outcome::Win,
            (Move::Paper, Move::Scissors) => Outcome::Loss,
            (Move::Rock, Move::Paper) => Outcome::Loss,
            (Move::Rock, Move::Scissors) => Outcome::Win,
            (Move::Scissors, Move::Paper) => Outcome::Win,
            (Move::Scissors, Move::Rock) => Outcome::Loss,
            _ => Outcome::Draw,
        }
    }
}

fn main() {
    let mut file = File::open("src/day02/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let games: Vec<(Move, Move)> = contents
        .split("\n")
        .map(|string| {
            let chars: Vec<&str> = string.split_whitespace().collect();
            let opponent = chars.get(0).and_then(|str| Move::for_opponent(*str));
            let player = chars.get(1).and_then(|str| Move::for_self(*str));
            player.zip(opponent)
        })
        .flatten()
        .collect();

    let mut score = 0;
    for (player, opponent) in games {
        score += player.value();
        score += Outcome::from_game(&player, &opponent).value();
    }

    println!("{}", score);
}
