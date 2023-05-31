use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

use petgraph::graph::NodeIndex;
use petgraph::visit::{EdgeRef, NodeIndexable, VisitMap, Visitable};
use petgraph::{Graph, Undirected};

pub fn dijkstra_all(graph: &Graph<(), u64, Undirected>, start: NodeIndex) -> Vec<u64> {
    let mut scores = vec![u64::MAX; graph.node_bound()];
    let mut visit_next = BinaryHeap::new();
    let mut visited = graph.visit_map();
    let start_score = 0;
    scores[start.index()] = start_score;
    visit_next.push(NoOrd(start_score, start));

    while let Some(NoOrd(_, node)) = visit_next.pop() {
        if !visited.visit(node) {
            continue;
        }
        for edge in graph.edges(node) {
            let next = edge.target();
            if visited.is_visited(&next) {
                continue;
            }
            let next_score = scores[node.index()] + edge.weight();
            let old_next_score = scores[next.index()];
            if next_score < old_next_score {
                scores[next.index()] = next_score;
                visit_next.push(NoOrd(next_score, next));
            }
        }
    }
    scores
}

pub fn dijkstra_single(
    graph: &Graph<(), u64, Undirected>,
    start: NodeIndex,
    end: NodeIndex,
) -> u64 {
    let mut scores = vec![u64::MAX; graph.node_bound()];
    let mut visit_next = BinaryHeap::new();
    let mut visited = graph.visit_map();
    let start_score = 0;
    scores[start.index()] = start_score;
    visit_next.push(NoOrd(start_score, start));

    while let Some(NoOrd(_, node)) = visit_next.pop() {
        if !visited.visit(node) {
            continue;
        }
        for edge in graph.edges(node) {
            let next = edge.target();
            if visited.is_visited(&next) {
                continue;
            }
            let next_score = scores[node.index()] + edge.weight();
            let old_next_score = scores[next.index()];
            if next_score < old_next_score {
                scores[next.index()] = next_score;
                visit_next.push(NoOrd(next_score, next));
            }
            if next == end {
                return next_score;
            }
        }
    }
    scores[end.index()]
}

#[derive(Copy, Clone, Debug)]
struct NoOrd<K, T>(K, T);

impl<K: PartialEq, T> PartialEq for NoOrd<K, T> {
    fn eq(&self, other: &NoOrd<K, T>) -> bool {
        self.0.eq(&other.0)
    }
}

impl<K: PartialEq, T> Eq for NoOrd<K, T> {}

impl<K: PartialOrd, T> PartialOrd for NoOrd<K, T> {
    fn partial_cmp(&self, other: &NoOrd<K, T>) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<K: PartialOrd, T> Ord for NoOrd<K, T> {
    fn cmp(&self, other: &NoOrd<K, T>) -> Ordering {
        other.0.partial_cmp(&self.0).unwrap()
    }
}

pub fn dial_all(
    graph: &Graph<(), u64, Undirected>,
    start_node: NodeIndex,
    max_cost: usize,
) -> Vec<Option<usize>> {
    let mut distance: Vec<_> = vec![None; graph.node_bound()];
    let mut buckets: Vec<_> = vec![Vec::new(); max_cost + 1];
    let mut in_bucket: Vec<_> = vec![false; graph.node_bound()];

    let mut visited = graph.visit_map();
    let start_index = graph.to_index(start_node);

    distance[start_index] = Some(0);
    buckets[0].push(start_index);
    in_bucket[start_index] = true;

    let mut i = 0;
    loop {
        let start = i;
        while buckets[i % (max_cost + 1)].is_empty() {
            i += 1;
            if i - start > max_cost {
                return distance;
            }
        }

        let node_index = buckets[i % (max_cost + 1)].pop().unwrap();

        let node_id = graph.from_index(node_index);
        in_bucket[node_index] = false;

        if visited.visit(node_id) {
            for edge in graph.edges(node_id) {
                let next_index = graph.to_index(edge.target());
                let new_cost = i + *edge.weight() as usize;

                if let Some(old_cost) = distance[next_index] {
                    if new_cost < old_cost {
                        if in_bucket[next_index] {
                            buckets[old_cost % (max_cost + 1)].retain(|&x| x != next_index)
                        }
                        buckets[new_cost % (max_cost + 1)].push(next_index);
                        in_bucket[next_index] = true;
                        distance[next_index] = Some(new_cost);
                    }
                } else {
                    buckets[new_cost % (max_cost + 1)].push(next_index);
                    in_bucket[next_index] = true;
                    distance[next_index] = Some(new_cost);
                }
            }
        }
    }
}

pub fn dial_single(
    graph: &Graph<(), u64, Undirected>,
    start_node: NodeIndex,
    end_node: NodeIndex,
    max_cost: usize,
) -> Option<usize> {
    let mut distance: Vec<_> = vec![None; graph.node_bound()];
    let mut buckets: Vec<_> = vec![Vec::new(); max_cost + 1];
    let mut in_bucket: Vec<_> = vec![false; graph.node_bound()];

    let mut visited = graph.visit_map();
    let start_index = graph.to_index(start_node);

    distance[start_index] = Some(0);
    buckets[0].push(start_index);
    in_bucket[start_index] = true;

    let mut i = 0;
    loop {
        let start = i;
        while buckets[i % (max_cost + 1)].is_empty() {
            i += 1;
            if i - start > max_cost {
                return distance[graph.to_index(end_node)];
            }
        }

        let node_index = buckets[i % (max_cost + 1)].pop().unwrap();

        let node_id = graph.from_index(node_index);
        in_bucket[node_index] = false;

        if visited.visit(node_id) {
            for edge in graph.edges(node_id) {
                let next_index = graph.to_index(edge.target());
                let next_id = graph.from_index(next_index);
                let new_cost = i + *edge.weight() as usize;

                if let Some(old_cost) = distance[next_index] {
                    if new_cost < old_cost {
                        if in_bucket[next_index] {
                            buckets[old_cost % (max_cost + 1)].retain(|&x| x != next_index);
                        }
                        buckets[new_cost % (max_cost + 1)].push(next_index);
                        in_bucket[next_index] = true;
                        distance[next_index] = Some(new_cost);
                    }
                } else {
                    buckets[new_cost % (max_cost + 1)].push(next_index);
                    in_bucket[next_index] = true;
                    distance[next_index] = Some(new_cost);
                }

                if next_id == end_node {
                    return distance[next_index];
                }
            }
        }
    }
}
