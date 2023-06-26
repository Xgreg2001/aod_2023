use clap::Parser;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::Graph;
use petgraph::Undirected;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

fn main() {
    let args = Cli::parse();

    let (graph, u, v) = generate_bipartite_graph(args.k, args.i);

    if let Some(path) = args.glpk {
        let file = std::fs::File::create(path).unwrap();

        let mut writer = std::io::BufWriter::new(file);

        write_lp_model(&mut writer, &graph, &u, &v);
    }

    let now = std::time::Instant::now();

    let matching = max_bipartite_matching(&graph, &u, &v, args.print_matching);

    let elapsed = now.elapsed();

    println!("Matching: {}", matching);
    println!("Time: {}", elapsed.as_millis());
}


fn write_lp_model(
    writer: &mut BufWriter<File>,
    graph: &Graph<u32, u32, Undirected>,
    u: &Vec<NodeIndex>,
    v: &Vec<NodeIndex>,
) {
    // print variables
    writeln!(writer, "/* Variables */").unwrap();
    for edge in graph.edge_references() {
        writeln!(
            writer,
            "var x_{}_{} binary;",
            edge.source().index(),
            edge.target().index()
        )
        .unwrap();
    }

    // print objective function
    writeln!(writer, "/* Objective function */").unwrap();
    let objectives: String = graph
        .edge_references()
        .map(|edge| format!("x_{}_{}", edge.source().index(), edge.target().index()))
        .collect::<Vec<String>>()
        .join(" + ");

    writeln!(writer, "maximize obj: {};", objectives).unwrap();

    // print constraints
    writeln!(writer, "/* Constraints */").unwrap();
    for node in u {
        let constraints: String = graph
            .edges(*node)
            .map(|edge| format!("x_{}_{}", edge.source().index(), edge.target().index()))
            .collect::<Vec<String>>()
            .join(" + ");
        if constraints.is_empty() {
            continue;
        }
        writeln!(writer, "s.t. c_{}: {} <= 1;", node.index(), constraints).unwrap();
    }

    writeln!(writer, "solve;").unwrap();
    writeln!(writer, "display {};", objectives).unwrap();

    writeln!(writer, "end;").unwrap();
}

fn bfs(
    graph: &Graph<u32, u32, Undirected>,
    match_u: &Vec<Option<NodeIndex>>,
    match_v: &Vec<Option<NodeIndex>>,
    u: &Vec<NodeIndex>,
    dist: &mut Vec<usize>,
) -> bool {
    let mut queue = VecDeque::new();
    for &node in u {
        if match_u[node.index()] == None {
            dist[node.index()] = 0;
            queue.push_back(node);
        } else {
            dist[node.index()] = usize::MAX;
        }
    }
    dist[u.len()] = usize::MAX;
    while let Some(node) = queue.pop_front() {
        if dist[node.index()] < dist[u.len()] {
            for edge in graph.edges(node) {
                let target = edge.target();
                if dist[match_v[target.index() - u.len()]
                    .unwrap_or(NodeIndex::new(u.len()))
                    .index()]
                    == usize::MAX
                {
                    dist[match_v[target.index() - u.len()]
                        .unwrap_or(NodeIndex::new(u.len()))
                        .index()] = dist[node.index()] + 1;
                    queue.push_back(
                        match_v[target.index() - u.len()].unwrap_or(NodeIndex::new(u.len())),
                    );
                }
            }
        }
    }
    dist[u.len()] != usize::MAX
}

fn dfs(
    node: NodeIndex,
    graph: &Graph<u32, u32, Undirected>,
    match_u: &mut Vec<Option<NodeIndex>>,
    match_v: &mut Vec<Option<NodeIndex>>,
    u: &Vec<NodeIndex>,
    dist: &mut Vec<usize>,
) -> bool {
    if node.index() == u.len() {
        return true;
    }
    for edge in graph.edges(node) {
        let target = edge.target();
        if dist[match_v[target.index() - u.len()]
            .unwrap_or(NodeIndex::new(u.len()))
            .index()]
            == dist[node.index()] + 1
            && dfs(
                match_v[target.index() - u.len()].unwrap_or(NodeIndex::new(u.len())),
                graph,
                match_u,
                match_v,
                u,
                dist,
            )
        {
            match_u[node.index()] = Some(target);
            match_v[target.index() - u.len()] = Some(node);
            return true;
        }
    }
    dist[node.index()] = usize::MAX;
    false
}

pub fn max_bipartite_matching(
    graph: &Graph<u32, u32, Undirected>,
    u: &Vec<NodeIndex>,
    v: &Vec<NodeIndex>,
    print: bool,
) -> usize {
    let mut match_u = vec![None; u.len()];
    let mut match_v = vec![None; v.len()];
    let mut dist = vec![0; u.len() + 1];

    let mut total_matches = 0;
    while bfs(graph, &match_u, &match_v, u, &mut dist) {
        for &node in u {
            if match_u[node.index()] == None
                && dfs(node, graph, &mut match_u, &mut match_v, u, &mut dist)
            {
                total_matches += 1;
            }
        }
    }

    if print {
        println!("Matching:");
        for (u, v_opt) in match_u.iter().enumerate() {
            if let Some(v) = v_opt {
                println!("Edge in matching: ({}, {})", u, v.index());
            }
        }
    }

    total_matches
}

pub fn generate_bipartite_graph(
    k: u32,
    i: usize,
) -> (Graph<u32, u32, Undirected>, Vec<NodeIndex>, Vec<NodeIndex>) {
    let mut rng = thread_rng();
    let size = 2usize.pow(k);

    // Create V1 and V2 as vectors from 0 to 2^k
    let mut u: Vec<NodeIndex> = Vec::with_capacity(size);
    let mut v: Vec<NodeIndex> = Vec::with_capacity(size);

    // Create an empty undirected Graph
    let mut graph: Graph<u32, u32, Undirected> = Graph::new_undirected();

    // Add vertices to the graph
    for _ in 0..size {
        u.push(graph.add_node(0));
    }
    for _ in 0..size {
        v.push(graph.add_node(0));
    }

    // for each node in V1, add i random edges to V2
    for &node in u.iter() {
        for &target in v.choose_multiple(&mut rng, i) {
            graph.add_edge(node, target, 0);
        }
    }

    (graph, u, v)
}

/// finding maximal matching in a bipartite graph
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long = "size")]
    k: u32,
    #[arg(long = "degree")]
    i: usize,
    #[arg(long = "printMatching")]
    print_matching: bool,
    #[arg(long = "glpk")]
    glpk: Option<PathBuf>,
}
