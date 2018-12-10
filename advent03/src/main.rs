use std::env;
use std::fs;

extern crate regex;

use regex::Regex;

fn main() {
    let filename = env::args().nth(1).expect("Usage: ./run FILENAME");
    let claims: Vec<String> = fs::read_to_string(filename)
        .expect("Unable to read file")
        .lines()
        .map(|l| l.parse().expect("Invalid claim in file"))
        .collect();
    println!("Part one: {}", part_one(&claims));
    println!("Part two: {}", part_two(&claims));
}

fn part_one(claims: &Vec<String>) -> u32 {
    let r = Regex::new(r"#.* @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)").unwrap();
    claims
        .iter()
        .map(|claim| {
            let caps = r.captures(claim).unwrap();
            let starting_x: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
            let starting_y: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
            let size_x: u32 = caps.get(3).unwrap().as_str().parse().unwrap();
            let size_y: u32 = caps.get(4).unwrap().as_str().parse().unwrap();

            (starting_x, starting_y, size_x, size_y)
        }).fold(
            vec![vec![0]],
            |mut acc, (starting_x, starting_y, size_x, size_y)| {
                while (acc.len() as u32) + 1 <= starting_y + size_y {
                    acc.push(vec![0]);
                }

                (starting_y..starting_y + size_y).for_each(|i| {
                    let row = &mut acc[i as usize];
                    while (row.len() as u32) + 1 <= starting_x + size_x {
                        row.push(0);
                    }
                    (starting_x..starting_x + size_x).for_each(|x| {
                        row[x as usize] += 1;
                    });
                });

                acc
            },
        ).iter()
        .map(|x| x.iter().filter(|y| **y > 1).count() as u32)
        .sum()
}

fn part_two(claims: &Vec<String>) -> u32 {
    let r = Regex::new(r"#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)").unwrap();
    let claims: Vec<(u32, u32, u32, u32, u32)> = claims
        .iter()
        .map(|claim| {
            let caps = r.captures(claim).unwrap();
            let id: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
            let starting_x: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
            let starting_y: u32 = caps.get(3).unwrap().as_str().parse().unwrap();
            let size_x: u32 = caps.get(4).unwrap().as_str().parse().unwrap();
            let size_y: u32 = caps.get(5).unwrap().as_str().parse().unwrap();

            (id, starting_x, starting_y, size_x, size_y)
        }).collect();

    let landscape = claims.iter().fold(
        vec![vec![0]],
        |mut acc, (_id, starting_x, starting_y, size_x, size_y)| {
            while (acc.len() as u32) + 1 <= starting_y + size_y {
                acc.push(vec![0]);
            }

            (*starting_y..*starting_y + *size_y).for_each(|i| {
                let row = &mut acc[i as usize];
                while (row.len() as u32) + 1 <= starting_x + size_x {
                    row.push(0);
                }
                (*starting_x..*starting_x + *size_x).for_each(|x| {
                    row[x as usize] += 1;
                });
            });

            acc
        },
    );

    *(claims
        .iter()
        .find_map(|(id, starting_x, starting_y, size_x, size_y)| {
            let mut overlap = false;
            for i in *starting_y..*starting_y + *size_y {
                let row = &landscape[i as usize];
                for j in *starting_x..*starting_x + *size_x {
                    if row[j as usize] > 1 {
                        overlap = true;
                    }
                    if overlap {
                        break;
                    }
                }
                if overlap {
                    break;
                }
            }

            if !overlap {
                return Some(id);
            };

            None
        }).unwrap())
}

#[cfg(test)]
mod tests {
    use part_one;
    use part_two;

    #[test]
    fn part_one_works() {
        let input = vec![
            "#1 @ 1,3: 4x4".to_string(),
            "#2 @ 3,1: 4x4".to_string(),
            "#3 @ 5,5: 2x2".to_string(),
        ];

        assert_eq!(4, part_one(&input))
    }

    #[test]
    fn part_two_works() {
        let input = vec![
            "#1 @ 1,3: 4x4".to_string(),
            "#2 @ 3,1: 4x4".to_string(),
            "#3 @ 5,5: 2x2".to_string(),
        ];

        assert_eq!(3, part_two(&input))
    }
}
