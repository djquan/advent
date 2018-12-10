use std::collections::HashSet;
use std::env;
use std::fs;
use std::iter::FromIterator;

fn main() {
    let filename = env::args().nth(1).expect("Usage: ./run FILENAME");
    let adjustments: Vec<i32> = fs::read_to_string(filename)
        .expect("Unable to read file")
        .lines()
        .map(|l| l.parse().expect("Invalid adjustment in file"))
        .collect();

    println!("Part one: {}", part_one(&adjustments));
    println!("Part two: {}", part_two(&adjustments));
}

pub fn part_one(adjustments: &Vec<i32>) -> i32 {
    adjustments.iter().sum()
}

pub fn part_two(adjustments: &Vec<i32>) -> i32 {
    let mut frequency = 0;
    let mut seen_frequencies: HashSet<i32> = HashSet::from_iter(vec![0]);
    loop {
        for adjustment in adjustments {
            frequency += adjustment;
            if seen_frequencies.contains(&frequency) {
                return frequency;
            }
            seen_frequencies.insert(frequency);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::part_one;
    use crate::part_two;

    #[test]
    fn part_one_works() {
        assert_eq!(3, part_one(vec![1, 1, 1].as_ref()));
        assert_eq!(0, part_one(vec![1, 1, -2].as_ref()));
        assert_eq!(-6, part_one(vec![-1, -2, -3].as_ref()));
    }

    #[test]
    fn part_two_works() {
        assert_eq!(0, part_two(vec![1, -1].as_ref()));
        assert_eq!(10, part_two(vec![3, 3, 4, -2, -4].as_ref()));
        assert_eq!(5, part_two(vec![-6, 3, 8, 5, -6].as_ref()));
        assert_eq!(14, part_two(vec![7, 7, -2, -7, -4].as_ref()));
    }
}
