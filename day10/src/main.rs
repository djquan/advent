extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::fs;

fn main() {
    let r = Regex::new(r"position=<(.*), (.*)> velocity=<(.*), (.*)>").unwrap();
    let input: Vec<(i32, i32, i32, i32)> = fs::read_to_string("input.txt")
        //    let input: Vec<(i32, i32, i32, i32)> = fs::read_to_string("test_input.txt")
        .expect("Unable to read file")
        .lines()
        .map(|l| {
            let captures = r.captures(l.trim()).unwrap();
            let x: i32 = captures.get(1).unwrap().as_str().trim().parse().unwrap();
            let y: i32 = captures.get(2).unwrap().as_str().trim().parse().unwrap();
            let w: i32 = captures.get(3).unwrap().as_str().trim().parse().unwrap();
            let z: i32 = captures.get(4).unwrap().as_str().trim().parse().unwrap();
            (x, y, w, z)
        })
        .collect();
    part_one(input, 20000);
}

fn part_one(input: Vec<(i32, i32, i32, i32)>, times: u32) {
    let mut input = input.clone();
    for i in 1..times {
        input = input
            .iter()
            .map(|(x, y, w, z)| (*x + *w, *y + *z, *w, *z))
            .collect();

        let (mut max_x, mut max_y, _, _) = input[0];
        let mut min_y = max_y;
        let mut min_x = max_x;

        let points: HashSet<(i32, i32)> = input
            .iter()
            .map(|(x, y, _, _)| {
                if *x > max_x {
                    max_x = *x;
                }

                if *x < min_x {
                    min_x = *x;
                }

                if *y > max_y {
                    max_y = *y;
                }

                if *y < min_y {
                    min_y = *y;
                }

                (*x, *y)
            })
            .collect();

        if max_x - min_x < 100 && max_y - min_y < 150 {
            println!("\nSecond : {}  ---------------------------------------------------------------------\n", i);
            for i in min_y..=max_y {
                for j in min_x..=max_x {
                    if points.contains(&(j, i)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!("\n");
            }
        }
    }
}
