extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use Event::Sleep;
use Event::Start;
use Event::Wake;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Event {
    Start(u32),
    Wake,
    Sleep,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Record {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    event: Event,
}

fn main() {
    let filename = env::args().nth(1).expect("Usage: ./run FILENAME");
    let records: Vec<String> = fs::read_to_string(filename)
        .expect("Unable to read file")
        .lines()
        .map(|l| l.parse().expect("Invalid records in file"))
        .collect();

    let records = parse(&records);

    println!("Part one: {}", part_one(&records));
    println!("Part two: {}", part_two(&records));
}

fn parse(input: &Vec<String>) -> HashMap<u32, HashMap<u32, u32>> {
    let mut x = input
        .iter()
        .map(|x| parse_record(x.clone()))
        .collect::<Vec<Record>>();
    x.sort_by_key(|f| (f.year, f.month, f.day, f.hour, f.minute));

    let mut current_guard = (0, 0);
    let mut tracker = HashMap::new();

    for record in x.iter() {
        match record.event {
            Event::Start(id) => current_guard = (id, 0),
            Event::Wake => {
                for i in current_guard.1..record.minute {
                    *tracker
                        .entry(current_guard.0)
                        .or_insert_with(HashMap::new)
                        .entry(i)
                        .or_insert(0) += 1;
                }
            }
            Event::Sleep => current_guard = (current_guard.0, record.minute),
        };
    }
    tracker
}

fn parse_record(input: String) -> Record {
    let r = Regex::new(r"\[([0-9]+)-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+)] (Guard #([0-9]+) begins shift|(falls asleep)|(wakes up))").unwrap();
    let caps = r.captures(input.as_str()).unwrap();
    let event = match caps.get(6).unwrap().as_str() {
        "falls asleep" => Sleep,
        "wakes up" => Wake,
        _ => Start(caps.get(7).unwrap().as_str().parse().unwrap()),
    };

    Record {
        year: caps.get(1).unwrap().as_str().parse().unwrap(),
        month: caps.get(2).unwrap().as_str().parse().unwrap(),
        day: caps.get(3).unwrap().as_str().parse().unwrap(),
        hour: caps.get(4).unwrap().as_str().parse().unwrap(),
        minute: caps.get(5).unwrap().as_str().parse().unwrap(),
        event,
    }
}

fn part_one(input: &HashMap<u32, HashMap<u32, u32>>) -> u32 {
    let (sleepy_guard, sleepy_times) = input
        .iter()
        .max_by_key(|(_, minutes)| minutes.values().sum::<u32>())
        .unwrap();

    let (sleepy_minute, _) = sleepy_times.iter().max_by_key(|&(_, count)| count).unwrap();

    sleepy_guard * sleepy_minute
}

fn part_two(input: &HashMap<u32, HashMap<u32, u32>>) -> u32 {
    let (sleepy_guard, sleepy_minute, _) = input
        .iter()
        .map(|(guard, values)| {
            let (sleepy_minute, count) = values.iter().max_by_key(|&(_, count)| count).unwrap();
            (guard, sleepy_minute, count)
        })
        .max_by_key(|&(_, _, count)| count)
        .unwrap();

    sleepy_guard * sleepy_minute
}

#[cfg(test)]
mod tests {
    use parse;
    use parse_record;
    use part_one;
    use part_two;
    use Event::Sleep;
    use Event::Start;
    use Event::Wake;
    use Record;

    #[test]
    fn parses_correctly() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift".to_string();
        let expected = Record {
            year: 1518,
            month: 11,
            day: 01,
            hour: 0,
            minute: 0,
            event: Start(10),
        };
        assert_eq!(expected, parse_record(input));

        let input = "[1518-11-01 00:05] falls asleep".to_string();
        let expected = Record {
            year: 1518,
            month: 11,
            day: 01,
            hour: 0,
            minute: 5,
            event: Sleep,
        };
        assert_eq!(expected, parse_record(input));

        let input = "[1518-11-01 00:25] wakes up".to_string();
        let expected = Record {
            year: 1518,
            month: 11,
            day: 01,
            hour: 0,
            minute: 25,
            event: Wake,
        };
        assert_eq!(expected, parse_record(input));
    }

    #[test]
    fn part_one_works() {
        let input = vec![
            "[1518-11-01 00:00] Guard #10 begins shift".to_string(),
            "[1518-11-01 00:05] falls asleep".to_string(),
            "[1518-11-01 00:25] wakes up".to_string(),
            "[1518-11-01 00:30] falls asleep".to_string(),
            "[1518-11-01 00:55] wakes up".to_string(),
            "[1518-11-01 23:58] Guard #99 begins shift".to_string(),
            "[1518-11-02 00:40] falls asleep".to_string(),
            "[1518-11-02 00:50] wakes up".to_string(),
            "[1518-11-03 00:05] Guard #10 begins shift".to_string(),
            "[1518-11-03 00:24] falls asleep".to_string(),
            "[1518-11-03 00:29] wakes up".to_string(),
            "[1518-11-04 00:02] Guard #99 begins shift".to_string(),
            "[1518-11-04 00:36] falls asleep".to_string(),
            "[1518-11-04 00:46] wakes up".to_string(),
            "[1518-11-05 00:03] Guard #99 begins shift".to_string(),
            "[1518-11-05 00:45] falls asleep".to_string(),
            "[1518-11-05 00:55] wakes up ".to_string(),
        ];
        assert_eq!(240, part_one(&parse(&input)));
    }

    #[test]
    fn part_two_works() {
        let input = vec![
            "[1518-11-01 00:00] Guard #10 begins shift".to_string(),
            "[1518-11-01 00:05] falls asleep".to_string(),
            "[1518-11-01 00:25] wakes up".to_string(),
            "[1518-11-01 00:30] falls asleep".to_string(),
            "[1518-11-01 00:55] wakes up".to_string(),
            "[1518-11-01 23:58] Guard #99 begins shift".to_string(),
            "[1518-11-02 00:40] falls asleep".to_string(),
            "[1518-11-02 00:50] wakes up".to_string(),
            "[1518-11-03 00:05] Guard #10 begins shift".to_string(),
            "[1518-11-03 00:24] falls asleep".to_string(),
            "[1518-11-03 00:29] wakes up".to_string(),
            "[1518-11-04 00:02] Guard #99 begins shift".to_string(),
            "[1518-11-04 00:36] falls asleep".to_string(),
            "[1518-11-04 00:46] wakes up".to_string(),
            "[1518-11-05 00:03] Guard #99 begins shift".to_string(),
            "[1518-11-05 00:45] falls asleep".to_string(),
            "[1518-11-05 00:55] wakes up ".to_string(),
        ];
        assert_eq!(4455, part_two(&parse(&input)));
    }
}
