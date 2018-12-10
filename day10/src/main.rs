use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).expect("Usage: ./run FILENAME");
    let input: Vec<String> = fs::read_to_string(filename)
        .expect("Unable to read file")
        .lines()
        .map(|l| l.parse().expect("Invalid data"))
        .collect();
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}

fn part_one() -> u32 {
    0
}

fn part_two() -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn part_one_works() {
        assert_eq!(0, part_one());
    }

    #[test]
    fn part_two_works() {
        assert_eq!(0, part_two())
    }
}
