use std::env;
use std::fs;

#[derive(PartialEq, Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

fn main() {
    let filename = env::args().nth(1).expect("Usage: ./run FILENAME");
    let mut records: Vec<u32> = fs::read_to_string(filename)
        .expect("Unable to read file")
        .trim()
        .to_string()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect();

    let parsed = parse(&mut records);

    println!("Part one {}", part_one(&parsed));
    println!("Part two {}", part_two(&parsed));
}

fn part_one(node: &Node) -> u32 {
    let sum: u32 = node.metadata.iter().sum();
    let children_sum: u32 = node.children.iter().map(|child| part_one(child)).sum();
    sum + children_sum
}

fn part_two(node: &Node) -> u32 {
    if node.children.len() == 0 {
        return node.metadata.iter().sum();
    }

    let node_sum: u32 = node.metadata.iter().map(|metadata| {
        if *metadata == 0 ||  *metadata as usize > node.children.len() {
            return 0;
        };
        return part_two(&node.children[*metadata as usize - 1]);
    }).sum();

    node_sum
}

fn parse(mut input: &mut Vec<u32>) -> Node {
    let mut node = Node {
        children: Vec::new(),
        metadata: Vec::new(),
    };


    let num_children = &input.remove(0);
    let num_metadata = &input.remove(0);

    for _ in 0..*num_children {
        node.children.push(parse(&mut input));
    }


    for _ in 0..*num_metadata {
        let metadata = &input.remove(0);
        node.metadata.push(*metadata);
    }

    node
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::part_one;
    use crate::part_two;
    use crate::Node;

    #[test]
    fn parses_correctly() {
        let mut input = vec![0, 2, 1, 2];
        let expected = Node {
            children: Vec::new(),
            metadata: vec![1, 2],
        };

        assert_eq!(expected, parse(&mut input));

        let mut input = vec![1, 0, 0, 2, 1, 2];
        let expected = Node {
            children: vec![Node {
                children: Vec::new(),
                metadata: vec![1, 2],
            }],
            metadata: Vec::new(),
        };

        assert_eq!(expected, parse(&mut input));

        let mut input = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
        let expected = Node {
            children: vec![
                Node { children: Vec::new(), metadata: vec![10, 11, 12] },
                Node {
                    children: vec![Node { children: Vec::new(), metadata: vec![99] }],
                    metadata: vec![2],
                }
            ],
            metadata: vec![1, 1, 2],
        };

        assert_eq!(expected, parse(&mut input));
    }

    #[test]
    fn part_one_works() {
        let mut input = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
        let parsed = parse(&mut input);

        assert_eq!(138, part_one(&parsed));
    }

    #[test]
    fn part_two_works() {
        let mut input = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
        let parsed = parse(&mut input);

        assert_eq!(66, part_two(&parsed));
    }
}