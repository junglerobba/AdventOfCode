use std::{fs::File, io::Read};

#[derive(Debug)]
struct Rucksack {
    compartment_01: Vec<char>,
    compartment_02: Vec<char>,
}

impl Rucksack {
    fn find_matches(&self) -> Option<char> {
        let mut matches: Vec<char> = self
            .compartment_01
            .iter()
            .filter(|e| self.compartment_02.contains(*e))
            .map(|e| e.to_owned())
            .collect();
        matches.dedup();
        return if matches.len() != 1 {
            None
        } else {
            matches.get(0).map(|e| e.clone())
        };
    }

    fn find_badge(bags: &[&str]) -> Option<char> {
        let mut result = None;
        for char in bags[0].chars() {
            if bags.iter().all(|bag| bag.contains(char)) {
                result = Some(char);
                break;
            };
        }
        result
    }
}

fn main() {
    let mut file = File::open("src/day03/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let items: Vec<Rucksack> = contents
        .lines()
        .map(|line| {
            let items: Vec<char> = line.chars().collect();
            let compartments: Vec<&[char]> = items.chunks(items.len() / 2).collect();
            Rucksack {
                compartment_01: compartments[0].into(),
                compartment_02: compartments[1].into(),
            }
        })
        .collect();

    let matches: Vec<char> = items
        .iter()
        .map(Rucksack::find_matches)
        .filter_map(|e| e)
        .collect();
    println!("{:?}", matches);

    let sum: u32 = matches.iter().map(get_char_value).sum();

    println!("{}", sum);

    // part 2

    let bags: Vec<&str> = contents.lines().collect();
    let sum: u32 = bags
        .chunks(3)
        .map(Rucksack::find_badge)
        .filter_map(|e| e)
        .map(|e| get_char_value(&e))
        .sum();

    println!("{}", sum);
}

fn get_char_value(char: &char) -> u32 {
    let offset = if char.is_ascii_lowercase() {
        96
    } else {
        64 - 26
    };
    let value = (*char as u32) - offset;
    value as u32
}
