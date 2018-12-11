use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    println!("{:?}", part_two(5153));
}

fn part_one(input: i32) -> (u32, u32) {
    let mut grid: Vec<Vec<i32>> = vec![];
    for y in 0..300 {
        let mut row = vec![];
        for x in 0..=300 {
            let rack_id = x + 1 + 10;
            let mut power_level: i32 = (y + 1) * rack_id + input;
            power_level *= rack_id;
            power_level = (power_level / 100) % 10;
            power_level -= 5;
            row.push(power_level);
        }
        grid.push(row);
    }

    let mut scores: HashMap<(u32, u32), i32> = HashMap::new();

    for y in 0..300 - 3 {
        for x in 0..300 - 3 {
            let mut score = 0;
            for x2 in x..x + 3 {
                for y2 in y..y + 3 {
                    score += grid[y2][x2];
                }
            }

            scores.insert((x as u32 + 1, y as u32 + 1), score);
        }
    }

    let ((x, y), score) = scores
        .iter()
        .max_by_key(|&(_, score)| score).unwrap();

    (*x, *y)
}

fn part_two(input: i32) -> (usize, usize, usize) {
    let mut grid: Vec<Vec<i32>> = vec![];
    for y in 0..300 {
        let mut row = vec![];
        for x in 0..=300 {
            let rack_id = x + 1 + 10;
            let mut power_level: i32 = (y + 1) * rack_id + input;
            power_level *= rack_id;
            power_level = (power_level / 100) % 10;
            power_level -= 5;
            row.push(power_level);
        }
        grid.push(row);
    }

    let mut scores: HashMap<(usize, usize, usize), i32> = HashMap::new();

    for i in 1..=300 {
        for y in 0..300 - i {
            for x in 0..300 - i {
                let mut score = 0;
                if x > 0 && y > 0 && i > 1 {
                    score = *(scores.entry((x, y, i - 1)).or_default());
                    for x2 in x..x + i {
                        score += grid[y + i - 1][x2];
                    }

                    for y2 in y..y + i {
                        score += grid[y2][x + i -1];
                    }
                } else {
                    for x2 in x..x + i {
                        for y2 in y..y + i {
                            score += grid[y2][x2];
                        }
                    }
                };

                scores.insert((x, y, i), score);
            }
        }
    }

    let ((x, y, size), score) = scores
        .iter()
        .max_by_key(|&(_, score)| score).unwrap();

    (*x + 1, *y + 1, *size)
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn part_one_works() {
        assert_eq!((33, 45), part_one(18));
        assert_eq!((21, 61), part_one(42));
        assert_eq!((5, 2), part_one(5153));
    }

    #[test]
    fn part_two_works() {
        assert_eq!((90, 269, 16), part_two(18));
    }
}
