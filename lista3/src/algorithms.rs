use std::cmp::Ordering;
use std::collections::BinaryHeap;
use petgraph::graph::NodeIndex;
use petgraph::visit::{IntoEdges, IntoNodeIdentifiers, VisitMap, Visitable};
use petgraph::algo::Measure;
use petgraph::Incoming;

pub fn dijkstra<G, F, K>(
    graph: G,
    start: NodeIndex,
    mut edge_cost: F,
) -> Vec<K>
    where
        G: IntoEdges + IntoNodeIdentifiers + Visitable,
        F: FnMut(G::EdgeRef, G::NodeId) -> K,
        K: Measure + Copy,
{
    let mut scores = vec![K::max(); graph.node_bound()];
    let mut visit_next = BinaryHeap::new();
    let mut visited = graph.visit_map();
    let start_score = K::default();
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
            let next_score = scores[node.index()] + edge_cost(edge, next);
            let old_next_score = scores[next.index()];
            if next_score < old_next_score {
                scores[next.index()] = next_score;
                visit_next.push(NoOrd(next_score, next));
            }
        }
    }
    scores
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