use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

extern crate regex;

fn main() {
    let r = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();
    let filename = env::args().nth(1).expect("Usage: ./run FILENAME");
    let records = fs::read_to_string(filename)
        .expect("Unable to read file")
        .lines()
        .map(|line| {
            let x = r.captures(line.trim()).unwrap();
            let y: char = x.get(1).unwrap().as_str().parse().unwrap();
            let z: char = x.get(2).unwrap().as_str().parse().unwrap();

            (y, z)
        })
        .collect();
    println!("Part one {}", part_one(&records));
    println!("Part two {}", part_two(&records, 5, 60));
}

fn part_one(input: &Vec<(char, char)>) -> String {
    let mut directions: HashMap<char, HashSet<char>> = HashMap::new();
    for (a, b) in input.iter() {
        directions.entry(*a).or_insert(HashSet::new()).insert(*b);

        directions.entry(*b).or_insert(HashSet::new());

        for (_, d) in &mut directions {
            if !d.contains(a) {
                continue;
            }

            d.insert(*b);
        }
    }

    let mut result = vec![];
    while directions.len() != 0 {
        let mut available = vec![];
        'outer: for (c, _) in directions.clone() {
            for (_, d) in directions.clone() {
                if d.contains(&c) {
                    continue 'outer;
                }
            }
            available.push(c);
        }

        available.sort();
        let choice = available[0];
        directions.remove(&choice);
        result.push(choice);
    }

    result.iter().collect()
}

fn part_two(input: &Vec<(char, char)>, num_workers: u32, additional_time: u32) -> u32 {
    let mut directions: HashMap<char, HashSet<char>> = HashMap::new();
    for (a, b) in input.iter() {
        directions.entry(*a).or_insert(HashSet::new()).insert(*b);

        directions.entry(*b).or_insert(HashSet::new());

        for (_, d) in &mut directions {
            if !d.contains(a) {
                continue;
            }

            d.insert(*b);
        }
    }

    let mut worker_queue: Vec<(char, u32)> = vec![];

    let mut time = 0;
    loop {
        let mut available = vec![];
        'outer: for (c, _) in directions.clone() {
            for (_, d) in directions.clone() {
                if d.contains(&c) {
                    continue 'outer;
                }
            }
            available.push(c);
        }
        available.sort();

        'available: for x in &available {
            if (worker_queue.len() as u32) < num_workers
                && worker_queue.iter().find(|(chr, _)| chr == x) == None
            {
                worker_queue.push((*x, (*x as u32) - 64 + additional_time));
            }
        }

        if worker_queue.len() == 0 {
            return time;
        }

        time += 1;

        for (c, count) in &mut worker_queue {
            if *count != 0 {
                *count -= 1;
            }

            if *count == 0 {
                directions.remove(c);
            }
        }

        worker_queue.retain(|(_, y)| *y != 0);
    }
}

#[cfg(test)]
mod tests {
    use crate::part_one;
    use crate::part_two;

    #[test]
    fn part_one_works() {
        let input = vec![
            ('C', 'A'),
            ('C', 'F'),
            ('A', 'B'),
            ('A', 'D'),
            ('B', 'E'),
            ('D', 'E'),
            ('F', 'E'),
        ];

        assert_eq!("CABDFE", part_one(&input))
    }

    #[test]
    fn part_two_works() {
        let input = vec![
            ('C', 'A'),
            ('C', 'F'),
            ('A', 'B'),
            ('A', 'D'),
            ('B', 'E'),
            ('D', 'E'),
            ('F', 'E'),
        ];

        assert_eq!(15, part_two(&input, 2, 0))
    }
}
