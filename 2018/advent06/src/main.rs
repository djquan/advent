use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).expect("Usage: ./run FILENAME");
    let records = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.split(',').collect())
        .map(|line: Vec<&str>| {
            let x = line[0].trim().parse().unwrap();
            let y = line[1].trim().parse().unwrap();
            (x, y)
        })
        .collect();

    println!("{}", part_one(records));
}

fn part_one(input: Vec<(i32, i32)>) -> i32 {
    let (max_x, _) = input.iter().max_by_key(|&(x, _)| x).unwrap();
    let (_, max_y) = input.iter().max_by_key(|&(_, y)| y).unwrap();
    let max_x = *max_x + 1;
    let max_y = *max_y + 1;

    let mut tracker = HashMap::new();
    let mut infinite_points: HashSet<&(i32, i32)> = HashSet::new();

    for x in 0..max_x {
        for y in 0..max_y {
            let mut points: Vec<(&(i32, i32), i32)> = input
                .iter()
                .map(|point| {
                    let distance = (point.0 - x).abs() + (point.1 - y).abs();
                    (point, distance)
                })
                .collect();

            points.sort_by_key(|&(_, distance)| distance);

            let (point1, distance1) = points[0];
            let (_, distance2) = points[1];

            if distance1 == distance2 {
                continue;
            }

            if x == 0 || x == max_x || y == 0 || y == max_y {
                infinite_points.insert(point1);
            }

            *tracker.entry(point1).or_insert(0) += 1;
        }
    }

    let mut v: Vec<i32> = tracker
        .iter()
        .filter(|(point, _)| !infinite_points.contains(*point))
        .map(|(_, distance)| *distance)
        .collect();

    v.sort_by_key(|&x| x);
    v.reverse();

    v[0]
}

#[cfg(test)]
mod tests {
    use crate::part_one;

    #[test]
    fn part_one_works() {
        let input = vec![(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)];

        assert_eq!(17, part_one(input));
    }
}
