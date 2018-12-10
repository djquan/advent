use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).expect("Usage: ./run FILENAME");
    let ids: Vec<String> = fs::read_to_string(filename)
        .expect("Unable to read file")
        .lines()
        .map(|l| l.parse().expect("Invalid input in file"))
        .collect();
    println!("Part 1: {}", part_one(&ids));
    println!("Part 2: {}", part_two(&ids));
}

pub fn part_two(ids: &Vec<String>) -> String {
    let (a, b) = differed_by_one(ids);
    let bytes_a = a.as_bytes();
    let bytes_b = b.as_bytes();
    let mut s = String::with_capacity(bytes_a.len() - 1);
    for i in 0..a.len() {
        if bytes_a[i] == bytes_b[i] {
            s.push(char::from(bytes_a[i]));
        }
    }
    s
}

pub fn differed_by_one(ids: &Vec<String>) -> (&str, &str) {
    for i in 0..ids.len() {
        for j in i + 1..ids.len() {
            let a = &ids[i];
            let b = &ids[j];
            if differ_by_one(a, b) {
                return (a, b);
            }
        }
    }
    ("", "")
}

pub fn differ_by_one(a: &str, b: &str) -> bool {
    let bytes_a = a.as_bytes();
    let bytes_b = b.as_bytes();
    let mut count = 0;
    for i in 0..a.len() {
        if bytes_a[i] != bytes_b[i] {
            count += 1;
        }

        if count > 1 {
            return false;
        }
    }
    count == 1
}

pub fn part_one(ids: &Vec<String>) -> u32 {
    let (t, th) =
        ids.iter()
            .map(letter_count)
            .fold((0, 0), |(mut two, mut three), (twos, threes)| {
                if twos == true {
                    two += 1;
                };
                if threes == true {
                    three += 1;
                }
                (two, three)
            });
    t * th
}

pub fn letter_count(s: &String) -> (bool, bool) {
    let counts = s.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    let mut two = false;
    let mut three = false;
    for (_c, count) in counts {
        match count {
            2 => two = true,
            3 => three = true,
            _ => (),
        }
        if (two, three) == (true, true) {
            return (true, true);
        }
    }
    (two, three)
}

#[cfg(test)]
mod tests {
    use differ_by_one;
    use differed_by_one;
    use letter_count;
    use part_one;
    use part_two;

    #[test]
    fn calculates_letter_count() {
        assert_eq!((false, false), letter_count(&"abcdef".to_string()));
        assert_eq!((true, true), letter_count(&"bababc".to_string()));
        assert_eq!((true, false), letter_count(&"abbcde".to_string()));
        assert_eq!((false, true), letter_count(&"abcccd".to_string()));
        assert_eq!((true, false), letter_count(&"aabcdd".to_string()));
        assert_eq!((true, false), letter_count(&"abcdee".to_string()));
        assert_eq!((false, true), letter_count(&"ababab".to_string()));
    }

    #[test]
    fn calculates_checksum() {
        let input = vec![
            "abcdef".to_string(),
            "bababc".to_string(),
            "abbcde".to_string(),
            "abcccd".to_string(),
            "aabcdd".to_string(),
            "abcdee".to_string(),
            "ababab".to_string(),
        ];

        assert_eq!(12, part_one(&input));
    }

    #[test]
    fn calculates_if_strings_differ_by_one() {
        assert_eq!(true, differ_by_one("abc", "abd"));
        assert_eq!(false, differ_by_one("abc", "abc"));
        assert_eq!(false, differ_by_one("abc", "xyz"));
    }

    #[test]
    fn calculates_correct_ids() {
        let input = vec![
            "abcde".to_string(),
            "fghij".to_string(),
            "klmno".to_string(),
            "pqrst".to_string(),
            "fguij".to_string(),
            "axcye".to_string(),
            "wvxyz".to_string(),
        ];

        assert_eq!(("fghij", "fguij"), differed_by_one(&input));
    }

    #[test]
    fn calculates_the_correct_answer_part_2() {
        let input = vec![
            "abcde".to_string(),
            "fghij".to_string(),
            "klmno".to_string(),
            "pqrst".to_string(),
            "fguij".to_string(),
            "axcye".to_string(),
            "wvxyz".to_string(),
        ];

        assert_eq!("fgij", part_two(&input));
    }
}
