use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, space1},
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::{terminated, tuple},
    IResult,
};
use rand::seq::{IteratorRandom, SliceRandom};
use rustc_hash::FxHashMap;

advent_of_code::solution!(25);

fn string_to_id(s: &str) -> u32 {
    let b = s.as_bytes();
    (b[0] - b'a') as u32 * 26 * 26 + (b[1] - b'a') as u32 * 26 + (b[2] - b'a') as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, connections) = parse_input(input).unwrap();

    // Determine min cut with Karger's algorithm
    // Alternative: Stoer–Wagner min cut algorithm - but this is not necessarily more efficient in unweighted graph

    // Graph represented as an adjacency list
    let mut graph = FxHashMap::default();
    for (from, _, to) in connections {
        let from_id = string_to_id(from);
        let mut to_ids = to.iter().map(|t| string_to_id(t)).collect::<Vec<u32>>();
        for &to_id in &to_ids {
            graph
                .entry(to_id)
                .and_modify(|v: &mut Vec<u32>| v.push(from_id))
                .or_insert(vec![from_id]);
        }
        graph
            .entry(from_id)
            .and_modify(|v| v.append(&mut to_ids))
            .or_insert(to_ids);
    }

    let mut rng = rand::thread_rng();

    loop {
        let mut merged_vertices = FxHashMap::default();

        let mut contracted_graph = graph.clone();
        while contracted_graph.len() > 2 {
            // pick a random edge (u, v)
            let &u = contracted_graph.keys().choose(&mut rng).unwrap();
            let &v = contracted_graph.get(&u).unwrap().choose(&mut rng).unwrap();

            // contract u & v into a single vertex
            let mut v_edges = contracted_graph.get(&v).unwrap().clone();
            v_edges.retain(|&adj| adj != u);
            for &edge in v_edges.iter() {
                // update adjacent nodes for v to remove v and add u
                // TODO: replace instead to avoid moving all elements
                contracted_graph.entry(edge).and_modify(|adj| {
                    adj.retain(|&e| e != v);
                    adj.push(u)
                });
            }
            // replace adjacent nodes of u by adding edges from v and removing v
            // TODO: can we replace instead of add/delete?
            contracted_graph.entry(u).and_modify(|adj| {
                adj.retain(|&e| e != v);
                adj.append(&mut v_edges);
            });
            // remove v from graph
            contracted_graph.remove(&v);

            merged_vertices
                .entry(u)
                .and_modify(|adj: &mut Vec<u32>| adj.push(v))
                .or_insert(vec![v]);
            if let Some(mut edges) = merged_vertices.remove(&v) {
                merged_vertices
                    .entry(u)
                    .and_modify(|adj| adj.append(&mut edges));
            }

            // _check_graph_integrity(&contracted_graph, v);
        }

        let (a, b) = contracted_graph.values().collect_tuple().unwrap();
        assert_eq!(a.len(), b.len());

        if a.len() == 3 {
            return contracted_graph
                .keys()
                .map(|k| merged_vertices.get(k).unwrap_or(&vec![]).len() as u32 + 1)
                .reduce(|acc, e| acc * e);
        }
    }
}

fn _check_graph_integrity(contracted_graph: &FxHashMap<u32, Vec<u32>>, v: u32) {
    for (&key, value) in contracted_graph {
        assert!(key != v);
        assert!(!value.contains(&v), "{key} should not contain {v}");
        for edge in value {
            assert!(
                contracted_graph.get(edge).unwrap().contains(&key),
                "{edge} should contain {key}"
            );
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<(&str, &str, Vec<&str>)>> {
    many1(terminated(
        tuple((alpha1, tag(": "), separated_list1(space1, alpha1))),
        opt(newline),
    ))(input)
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }
}
