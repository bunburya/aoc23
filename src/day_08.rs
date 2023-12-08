use std::collections::HashMap;
use std::str::{FromStr, Lines};
use num::integer::lcm;

#[derive(Clone, Debug)]
struct Node {
    label: String,
    left: String,
    right: String
}

impl<'a> FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let label = String::from(&s[0..3]);
        let left = String::from(&s[7..10]);
        let right = String::from(&s[12..15]);
        Ok(Node { label, left, right })
    }
}

fn get_nodes(lines: Lines) -> (HashMap<String, usize>, Vec<Node>) {
    let mut node_map: HashMap<String, usize> = HashMap::new();
    let mut node_vec: Vec<Node> = vec!();
    for (node_i, line) in lines.enumerate() {
        let n = Node::from_str(line)
            .expect(&format!("Error parsing Node from line `{line}`."));
        let n_label = n.label.clone();
        node_vec.push(n);
        node_map.insert(n_label, node_i);
    }
    (node_map, node_vec)
}

fn get_steps(
    instructions: &str,
    start_node: &Node,
    node_map: &HashMap<String, usize>,
    node_vec: &[Node],
    end_fn: fn(&str) -> bool
) -> i64 {
    let mut current_node = start_node;
    let mut steps = 0;
    loop {
        for c in instructions.chars() {
            steps += 1;
            let label = match c {
                'L' => current_node.left.clone(),
                'R' => current_node.right.clone(),
                _ => panic!("Invalid instruction.")
            };
            if end_fn(&label) {
                return steps;
            } else {
                current_node = &node_vec[node_map[&label]];
            }
        }
    }

}

pub(crate) fn part_1(s: &str) -> String {
    let mut lines = s.lines();
    let instructions = lines.next().expect("Could not find instruction line.");
    lines.next();
    let (node_map, node_vec) = get_nodes(lines);
    get_steps(
        instructions,
        &node_vec[node_map["AAA"]],
        &node_map,
        &node_vec,
        |s| s == "ZZZ"
    ).to_string()
}

pub(crate) fn part_2(s: &str) -> String {
    let mut lines = s.lines();
    let instructions = lines.next().expect("Could not find instruction line.");
    lines.next();
    let (node_map, node_vec) = get_nodes(lines);
    let mut current_nodes: Vec<&Node> = vec!();
    for n in node_vec.iter() {
        if n.label.ends_with('A') {
            current_nodes.push(n);
        }
    }
    current_nodes.iter()
        .map(|&n| get_steps(
            instructions,
            n,
            &node_map,
            &node_vec,
            |s| s.ends_with('Z')
        ))
        .fold(1, lcm)
        .to_string()
}