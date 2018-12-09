use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).expect("Usage: ./run FILENAME");
    let records = fs::read_to_string(filename)
        .expect("Unable to read file");


    println!("Part 1: {}", part_one(records.trim()));
    println!("Part 2: {}", part_two(records.trim()));
}

fn part_one(seq: &str) -> u32 {
    let mut chars: Vec<char> = seq.chars().collect();

    'outer: loop {
        let mut new_chars = vec![];
        for i in 0..chars.len() - 1 {
            let c = chars[i];
            let next = chars[i + 1];
            if c != next && c.to_ascii_lowercase() == next.to_ascii_lowercase() {
                new_chars.extend_from_slice(&mut chars[i + 2..]);
                chars = new_chars;
                continue 'outer;
            }

            new_chars.push(c);
        }
        break;
    }

    chars.len() as u32
}

fn part_two(seq: &str) -> u32 {
    let chars: Vec<char> = seq.chars().collect();
    (b'a'..b'z' + 1)
        .map(|x| x as char)
        .map(|x| chars
            .iter()
            .filter(move |c| **c != x && c.to_ascii_lowercase() != x)
            .collect())
        .map(|x: String| {
            part_one(x.as_str())
        })
        .min()
        .unwrap() as u32
}


#[cfg(test)]
mod tests {
    use crate::part_one;
    use crate::part_two;

    #[test]
    fn parses_correctly() {
        assert_eq!(10, part_one("dabAcCaCBAcCcaDA"));
    }

    #[test]
    fn parses_part_two_correctly() {
        assert_eq!(4, part_two("dabAcCaCBAcCcaDA"));
    }
}
