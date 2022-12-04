use std::{fs::File, io::Read};

fn main() {
    let elves = get_elves();

    day_one_part_one(&elves);

    day_one_part_two(&elves)
}

fn day_one_part_one(elves: &Vec<Vec<i32>>) {
    let highest = elves
        .iter()
        .map(|elf| elf.iter().sum::<i32>())
        .max()
        .unwrap();

    println!("{:?}", highest);
}

fn day_one_part_two(elves: &Vec<Vec<i32>>) {
    let mut elves: Vec<i32> = elves.iter().map(|elf| elf.iter().sum::<i32>()).collect();

    elves.sort();
    elves.reverse();

    let total = &elves[..=2].iter().sum::<i32>();

    println!("{:?}, {:?}, {:?}", elves.get(0), elves.get(1), elves.get(2));

    println!("{:?}", total);
}

fn get_elves() -> Vec<Vec<i32>> {
    let mut file = File::open("src/day01/values").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");

    contents.split("\n\n").map(get_elf).collect()
}

fn get_elf(value: &str) -> Vec<i32> {
    value
        .split("\n")
        .map(|split| split.parse::<i32>())
        .filter_map(|value| value.ok())
        .collect()
}
