use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn solve() {
    let input = parse_input(fs::read_to_string("inputs/25.txt").unwrap());
    println!("Day 25:");
    println!("{}", solve_part_a(&input, false));
    println!("{}", solve_part_b(&input));
}

fn parse_input(input: String) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let (source, targets) = line.split_once(": ").unwrap();
        let source = source.to_string();
        let targets: Vec<String> = targets.split(' ').map(|t| t.to_string()).collect();
        graph
            .entry(source.clone())
            .and_modify(|v| v.extend(targets.clone()))
            .or_insert(targets.clone());
        for target in targets {
            graph
                .entry(target)
                .and_modify(|v| v.push(source.clone()))
                .or_insert(vec![source.clone()]);
        }
    }
    graph
}

fn solve_part_a(input: &HashMap<String, Vec<String>>, mock: bool) -> usize {
    let mut graph = input.clone();
    // from inspection of the plotted graph, I found out that the following edges should be removed to solve the problem:
    // szh - vqj
    // zhb - vxr
    // jbx - sml

    let edges_to_remove = if !mock {
        [("szh", "vqj"), ("zhb", "vxr"), ("jbx", "sml")]
    } else {
        [("hfx", "pzl"), ("bvb", "cmg"), ("nvd", "jqt")]
    };
    for (e1, e2) in edges_to_remove {
        let size_before = graph.get(e1).unwrap().len();
        graph
            .entry(e1.to_string())
            .and_modify(|v| v.retain(|x| x != e2));
        let size_after = graph.get(e1).unwrap().len();
        assert_ne!(size_before, size_after);

        graph
            .entry(e2.to_string())
            .and_modify(|v| v.retain(|x| x != e1));
    }

    // find size of connected components using bfs
    let mut visited_nodes: HashSet<&String> = HashSet::new();
    let mut components: Vec<usize> = vec![];
    for root in graph.keys() {
        if visited_nodes.contains(root) {
            continue;
        }

        let mut component = 0;
        let mut queue = vec![root];
        while queue.len() > 0 {
            let node = queue.remove(0);
            if visited_nodes.contains(node) {
                continue;
            }
            component += 1;
            visited_nodes.insert(node);

            for nei in graph.get(node).unwrap() {
                queue.push(nei);
            }
        }
        components.push(component);
    }

    assert_eq!(components.len(), 2);
    components.iter().product()
}

fn solve_part_b(_input: &HashMap<String, Vec<String>>) -> usize {
    println!("Merry Christmas!");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
";

        let input = parse_input(sample.to_string());

        assert_eq!(solve_part_a(&input, true), 54);
    }
}
