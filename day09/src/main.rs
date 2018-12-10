use std::collections::HashMap;

fn main() {
    println!("{}", part_one(416, 71975 * 100));
}

fn part_one(players: u32, last_marble_score: u32) -> u32 {
    let mut current_player = 0;
    let mut circle = vec![0];
    let mut current_marble_value = 0;
    let mut current_marble_index = 0;
    let mut score: HashMap<u32, u32> = HashMap::new();

    while current_marble_value < last_marble_score {
        current_player = (current_player + 1) % players;
        current_marble_value += 1;

        if current_marble_value % 23 == 0 {
            *score.entry(current_player).or_insert(0) += current_marble_value;
            for _ in 0..7 {
                if current_marble_index == 0 {
                    current_marble_index = circle.len();
                }
                current_marble_index -= 1;
            }

            *score.entry(current_player).or_insert(0) += circle.remove(current_marble_index);
        } else {
            current_marble_index = (current_marble_index + 2) % circle.len();
            circle.insert(current_marble_index, current_marble_value);
        }
    }

    let (_, high_score) = score.iter().max_by_key(|&(_, y)| y).unwrap();

    *high_score
}

#[cfg(test)]
mod test {
    use crate::part_one;

    #[test]
    fn part_one_works() {
        assert_eq!(32, part_one(9, 23));
        assert_eq!(8317, part_one(10, 1618));
        assert_eq!(146373, part_one(13, 7999));
        assert_eq!(2764, part_one(17, 1104));
        assert_eq!(54718, part_one(21, 6111));
        assert_eq!(37305, part_one(30, 5807));
        assert_eq!(439341, part_one(416, 71975));
        assert_eq!(439341, part_one(416, 71975 * 100));
    }
}
