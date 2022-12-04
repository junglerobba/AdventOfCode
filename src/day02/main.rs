use std::{fs::File, io::Read};

#[derive(PartialEq, Debug, Copy, Clone)]
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

    fn from_letter(string: &str) -> Option<Outcome> {
        match string {
            "X" => Some(Outcome::Loss),
            "Y" => Some(Outcome::Draw),
            "Z" => Some(Outcome::Win),
            _ => None,
        }
    }

    fn get_player_move(&self, opponent: &Move) -> Move {
        match (self, opponent) {
            (Outcome::Draw, _) => opponent.clone(),
            (Outcome::Win, Move::Rock) => Move::Paper,
            (Outcome::Win, Move::Paper) => Move::Scissors,
            (Outcome::Win, Move::Scissors) => Move::Rock,
            (Outcome::Loss, Move::Rock) => Move::Scissors,
            (Outcome::Loss, Move::Paper) => Move::Rock,
            (Outcome::Loss, Move::Scissors) => Move::Paper,
        }
    }
}

fn main() {
    // part 01
    let mut file = File::open("src/day02/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let games: Vec<(Move, Move)> = contents
        .split("\n")
        .map(|string| {
            let chars: Vec<&str> = string.split_whitespace().collect();
            let opponent = chars.get(0).and_then(|str| Move::for_opponent(*str));
            let player = chars.get(1).and_then(|str| Move::for_self(*str));
            opponent.zip(player)
        })
        .flatten()
        .collect();

    let mut score = 0;
    for (opponent, player) in games {
        score += player.value();
        score += Outcome::from_game(&player, &opponent).value();
    }

    println!("{}", score);

    // part 02
    let games: Vec<(Move, Outcome)> = contents
        .split("\n")
        .map(|string| {
            let chars: Vec<&str> = string.split_whitespace().collect();
            let opponent = chars.get(0).and_then(|str| Move::for_opponent(*str));
            let outcome = chars.get(1).and_then(|str| Outcome::from_letter(*str));
            opponent.zip(outcome)
        })
        .flatten()
        .collect();

    let mut score = 0;
    for (opponent, outcome) in games {
        score += outcome.get_player_move(&opponent).value();
        score += outcome.value();
    }

    println!("{:?}", score);
}
