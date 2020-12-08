use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve() {
    let input = include_str!("../input/day7");
    let graph = Graph::from_rules(input);

    println!("Part 1: {}", part1(&graph));
    println!("Part 2: {}", part2(&graph));
}

fn part1(graph: &Graph) -> u32 {
    // Start with all the bags that directly contain a shiny gold bag
    let mut outer_bags: HashSet<u32> = graph.outer_bags(graph.node("shiny gold")).collect();

    loop {
        let mut layer = HashSet::new();

        for &bag in outer_bags.iter() {
            layer.extend(graph.outer_bags(bag));
        }

        if layer.difference(&outer_bags).count() == 0 {
            break;
        }

        outer_bags.extend(layer);
    }

    outer_bags.len() as u32
}

fn part2(graph: &Graph) -> u32 {
    graph.count_inner_bags(graph.node("shiny gold"))
}

type NodeID = u32;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Edge {
    outer: NodeID,
    inner: NodeID,
    count: u32,
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<String, NodeID>,
    edges: Vec<Edge>,
}

impl Graph {
    fn from_rules(rules: &str) -> Graph {
        let outer_re = Regex::new(r"^(\w+ \w+) bags contain ([\w ,]+)\.$").unwrap();
        let inner_re = Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();

        let mut graph = Graph {
            nodes: HashMap::new(),
            edges: Vec::new(),
        };

        for line in rules.lines() {
            let m = outer_re.captures(line).unwrap();
            let outer = graph.insert_node(m.get(1).unwrap().as_str());

            for m in inner_re.captures_iter(m.get(2).unwrap().as_str()) {
                let count = m.get(1).unwrap().as_str().parse().unwrap();
                let inner = graph.insert_node(m.get(2).unwrap().as_str());
                graph.insert_edge(outer, inner, count);
            }
        }

        graph
    }

    fn insert_node(&mut self, node: &str) -> NodeID {
        let next_node_id = self.nodes.len() as u32;
        *self.nodes.entry(node.to_string()).or_insert(next_node_id)
    }

    fn node(&self, node: &str) -> NodeID {
        *self.nodes.get(node).unwrap()
    }

    fn insert_edge(&mut self, outer: NodeID, inner: NodeID, count: u32) {
        self.edges.push(Edge {
            outer,
            inner,
            count,
        });
    }

    fn outer_bags(&self, inner: NodeID) -> impl Iterator<Item = NodeID> + '_ {
        self.edges.iter().filter_map(move |e| {
            if e.inner == inner {
                Some(e.outer)
            } else {
                None
            }
        })
    }

    fn inner_bags(&self, outer: NodeID) -> impl Iterator<Item = (u32, NodeID)> + '_ {
        self.edges.iter().filter_map(move |e| {
            if e.outer == outer {
                Some((e.count, e.inner))
            } else {
                None
            }
        })
    }

    fn count_inner_bags(&self, outer_bag: NodeID) -> u32 {
        let mut total_count = 0;

        for (count, inner_bag) in self.inner_bags(outer_bag) {
            total_count += count * (1 + self.count_inner_bags(inner_bag));
        }

        total_count
    }
}
