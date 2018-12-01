use std::collections::HashSet;
use std::iter::FromIterator;

pub fn frequency1(fs: Vec<i32>) -> i32 {
    fs.iter().sum()
}

pub fn frequency2(fs: Vec<i32>) -> i32 {
    let mut freq = 0;
    let mut seen_freqs: HashSet<i32> = HashSet::from_iter(vec![0]);
    loop {
        for f in &fs {
            freq += f;
            if seen_freqs.contains(&freq) {
                return freq;
            }
            seen_freqs.insert(freq);
        }
    }
}

#[cfg(test)]
mod tests {
    use frequency1;
    use frequency2;

    #[test]
    fn part_one() {
        assert_eq!(3, frequency1(vec![1, 1, 1]));
        assert_eq!(0, frequency1(vec![1, 1, -2]));
        assert_eq!(-6, frequency1(vec![-1, -2, -3]));
    }

    #[test]
    fn part_two() {
        assert_eq!(0, frequency2(vec![1, -1]));
        assert_eq!(10, frequency2(vec![3, 3, 4, -2, -4]));
        assert_eq!(5, frequency2(vec![-6, 3, 8, 5, -6]));
        assert_eq!(14, frequency2(vec![7, 7, -2, -7, -4]));
    }
}
