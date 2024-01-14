use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
struct Node {
    id: String,
    neighbors: BTreeSet<String>,
    weight: u64,
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    let (rem, (id, neighbors)) =
        separated_pair(alpha1, tag(": "), separated_list1(tag(" "), alpha1))(input)?;
    Ok((
        rem,
        Node {
            id: String::from(id),
            neighbors: neighbors.iter().map(|&s| String::from(s)).collect(),
            weight: 0,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, HashMap<String, Node>> {
    let (rem, nodes) = separated_list1(tag("\n"), parse_node)(input)?;
    let mut graph: HashMap<String, Node> =
        nodes.iter().map(|m| (m.id.clone(), m.clone())).collect();
    for node in nodes {
        for neighbor in &node.neighbors {
            if let Some(nnode) = graph.get_mut(neighbor) {
                nnode.neighbors.insert(node.id.clone());
            } else {
                let mut neighbors = BTreeSet::new();
                neighbors.insert(node.id.clone());
                graph.insert(
                    neighbor.clone(),
                    Node {
                        id: neighbor.clone(),
                        neighbors,
                        weight: 0,
                    },
                );
            }
        }
    }

    Ok((rem, graph))
}

fn get_graph_size(
    graph: &HashMap<String, Node>,
    skip_edges: &BTreeSet<(&String, &String)>,
) -> usize {
    let mut visited = BTreeSet::new();
    let init = graph.values().next().unwrap();
    let mut stack = VecDeque::new();
    stack.push_back(init);
    let mut ret = 0;
    while let Some(node) = stack.pop_front() {
        if visited.contains(node) {
            continue;
        }
        visited.insert(node);
        for n in &node.neighbors {
            if skip_edges.contains(&(&n, &node.id)) {
                continue;
            }
            if skip_edges.contains(&(&node.id, &n)) {
                continue;
            }
            stack.push_back(graph.get(n).unwrap());
        }
        ret += 1;
    }
    ret
}

fn get_graph_edges(graph: &HashMap<String, Node>) -> HashMap<(String, String), u64> {
    let mut visited = BTreeSet::new();
    let init = graph.values().next().unwrap();
    let mut stack = VecDeque::new();
    let mut weights = HashMap::new();
    stack.push_back(init);
    while let Some(node) = stack.pop_front() {
        if visited.contains(node) {
            continue;
        }
        visited.insert(node);
        for n in &node.neighbors {
            stack.push_back(graph.get(n).unwrap());
            let l_id = node.id.clone();
            let r_id = n.clone();
            let key = (l_id, r_id);
            if weights.contains_key(&key) {
                continue;
            }
            let key = (n.clone(), node.id.clone());
            if weights.contains_key(&key) {
                continue;
            }
            weights.insert(key, 0);
        }
    }
    weights
}

/**
*   For each node start a BFS. After having visited all possible nodes
*   append the furthest step taken as a node to each of its edges.
*
*   This function provides a way to sort the graph so that edges that are
*   more likely to be cut edges (edges of nodes that are closer to all other nodes)
*   are checked first.
*/
fn generate_furthest_node_weights(
    graph: &mut HashMap<String, Node>,
    weights: &mut HashMap<(String, String), u64>,
) {
    let keys: Vec<&String> = graph.keys().collect();
    for startnode in keys {
        let mut heap = VecDeque::new();
        let mut visited = HashSet::new();

        for neigh in &graph[startnode].neighbors {
            heap.push_front((0, (startnode, neigh)));
        }

        let mut max_cost = 0;
        while let Some((cost, (lnode, rnode))) = heap.pop_back() {
            if visited.contains(&(lnode, rnode)) {
                continue;
            }
            visited.insert((lnode, rnode));
            for neighbor in &graph.get(rnode).unwrap().neighbors {
                heap.push_front((cost + 1, (rnode, neighbor)));
            }
            max_cost = cost;
        }
        for neigh in &graph[startnode].neighbors {
            let l = startnode.to_string();
            let r = neigh.to_string();
            if weights.get(&(l.to_string(), r.to_string())).is_some() {
                weights.insert((l, r), max_cost);
            } else if weights.get(&(r.to_string(), l.to_string())).is_some() {
                weights.insert((r, l), max_cost);
            } else {
                println!("edge not found {} {}", l, r);
            }
        }
    }
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string(path)?.trim().parse()?;
    let (rem, mut graph) = parse_input(&input).unwrap();
    if !rem.is_empty() {
        panic!("Remaining input {}", rem);
    }
    let initsize = get_graph_size(&graph, &BTreeSet::new());
    println!("Initial graph size: {}", initsize);
    let mut weights: HashMap<(String, String), u64> = get_graph_edges(&graph);
    generate_furthest_node_weights(&mut graph, &mut weights);
    let mut v: Vec<_> = weights.into_iter().collect();
    v.sort_by(|(_, a), (_, b)| a.cmp(b));
    let ve: Vec<(String, String)> = v.into_iter().map(|(e, _)| e).collect();
    let mut visited: BTreeSet<(usize, usize, usize)> = BTreeSet::new();
    // Remove 3 edges at a time testing if the graph is split in two.
    // Search for a solution starting from the edges more likely to be cut-edges.
    for cnt in 6..ve.len() {
        for i in 0..cnt {
            for j in (i + 1)..cnt {
                for k in (j + 1)..cnt {
                    if visited.contains(&(i, j, k)) {
                        continue;
                    } else {
                        visited.insert((i, j, k));
                    }
                    let (li, ri) = &ve[j];
                    let (lj, rj) = &ve[i];
                    let (lk, rk) = &ve[k];
                    let mut skip_edges = BTreeSet::new();
                    skip_edges.insert((li, ri));
                    skip_edges.insert((lj, rj));
                    skip_edges.insert((lk, rk));
                    let size = get_graph_size(&graph, &skip_edges);
                    if size != initsize {
                        println!("{} {} {}", i, j, k);
                        println!( "{} * {} = {}", size, initsize - size, size * (initsize - size));
                        return Ok(());
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn pt2(_path: String) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
