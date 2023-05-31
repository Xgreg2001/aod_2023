use petgraph::algo::Measure;
use petgraph::graph::{Node, NodeIndex};
use petgraph::visit::{
    EdgeRef, IntoEdges, IntoNodeIdentifiers, NodeIndexable, VisitMap, Visitable,
};
use petgraph::{Graph, Incoming, Undirected};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

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
