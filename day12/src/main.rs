extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let input = vec![
        "##...",
        "..#.#",
        "#..#.",
        "#.##.",
        ".#.##",
        ".##..",
        "..#..",
        "#..##",
        "#.#..",
        "..###",
        "...#.",
        "##.#.",
        "#.#.#",
        "#####",
        ".#.#.",
        ".###.",
        "###.#",
    ];
    let initial_state = "#.##.#.##..#.#...##...#......##..#..###..##..#.#.....##..###...#.#..#...######...#####..##....#..###";

    println!("Part one: {}", part_one(initial_state, &input));
//    println!("Part two: {}", part_two());
}

fn part_one(initial_state: &str, commands: &Vec<&str>) -> isize {
    let commands: HashSet<&str> = HashSet::from_iter(commands.iter().cloned());
    let mut state: Vec<char> = initial_state.to_string().chars().collect();
    let mut hash_state: HashSet<isize> = HashSet::new();
    for i in 0..state.len() {
        if state[i] == '#' {
            hash_state.insert(i as isize);
        }
    };

    let mut sum = 0;
    for i in 0..20 as i64 {
//    for i in 0..50_000_000_000 as i64 {
        if i % 1_000_000_000 == 0 {
            println!("{}", i);
        }
        let start = *hash_state.iter().min().unwrap() - 1;
        let end = *hash_state.iter().max().unwrap() + 1;
        let mut next_gen: HashSet<isize> = HashSet::new();

        for j in start..=end {
            let target: String = (j - 2..=j + 2)
                .map(|t| hash_state.get(&t).map(|_| '#').unwrap_or('.'))
                .collect();

            if commands.contains(target.as_str()) {
                next_gen.insert(j);
            }
        }


        hash_state = next_gen;
        let new_sum: isize = hash_state.iter().sum::<isize>();
        println!("{}, {}", sum, new_sum);
        if sum == new_sum {
            return new_sum;
        }
        sum = new_sum;
    }

    sum
}

fn part_two() -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};
    use regex::Regex;

    #[test]
    fn part_one_works() {
        let input = vec![
            "...##",
            "..#..",
            ".#...",
            ".#.#.",
            ".#.##",
            ".##..",
            ".####",
            "#.#.#",
            "#.###",
            "##.#.",
            "##.##",
            "###..",
            "###.#",
            "####.",
        ];

        assert_eq!(325, part_one("#..#.#..##......###...###", &input));
    }

    #[test]
    #[ignore]
    fn part_two_works() {
        for i in 0..(50_000_000_000 as u64) {
//    for i in 0..(20 as u64) {
            if i % 1_000_000_000 == 0 {
                println!("{}", i);
            }
        }
    }
}
