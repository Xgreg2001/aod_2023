use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use log::debug;
use petgraph::graph::{node_index, NodeIndex};
use petgraph::graphmap::DiGraphMap;
use petgraph::visit::Visitable;
use rand::Rng;
use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::Duration;

fn main() {
    env_logger::init();

    let cli = Args::parse();

    let k = cli.k;
    println!("k: {}", k);

    let mut graph = initialize_graph(k);

    if let Some(path) = cli.glpk {
        let file = std::fs::File::create(path).unwrap();

        let mut writer = std::io::BufWriter::new(file);

        write_lp_model(&mut writer, &graph, k);
    }

    let now = std::time::Instant::now();
    let (flow, paths) = edmonds_karp(&mut graph, 0, 2usize.pow(k as u32) - 1);
    let elapsed = now.elapsed();

    if cli.print_flow {
        print_flows(&graph);
    }

    println!("Max flow: {}", flow);
    println!("Augmenting paths: {}", paths);
    println!(
        "Elapsed: {}.{:03}s",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );
}

fn write_lp_model(writer: &mut BufWriter<File>, graph: &DiGraphMap<usize, Edge>, k: usize) {
    // print variables
    writeln!(writer, "/* Variables */").unwrap();
    for edge in graph.all_edges() {
        if edge.2.cap != 0 {
            writeln!(writer, "var x_{}_{} >= 0;", edge.0, edge.1).unwrap();
        }
    }

    // print objective function
    writeln!(writer, "/* Objective function */").unwrap();
    let objectives: String = graph
        .edges(0)
        .filter(|edge| edge.2.cap != 0)
        .map(|edge| format!("x_{}_{}", edge.0, edge.1))
        .collect::<Vec<String>>()
        .join(" + ");
    writeln!(writer, "maximize obj: {};", objectives).unwrap();

    // print constraints
    writeln!(writer, "/* Constraints */").unwrap();

    // Flow capacity constraints
    writeln!(writer, "/* Flow capacity constraints */").unwrap();
    for node in graph.nodes() {
        for edge in graph.edges(node) {
            if edge.2.cap != 0 {
                writeln!(
                    writer,
                    "s.t. c_{}_{}: x_{}_{} <= {};",
                    edge.0, edge.1, edge.0, edge.1, edge.2.cap
                )
                .unwrap();
            }
        }
    }

    // Flow conservation constraints
    writeln!(writer, "/* Flow conservation constraints */").unwrap();
    for node in graph.nodes() {
        let incoming: String = graph
            .edges_directed(node, petgraph::Direction::Incoming)
            .filter(|edge| edge.2.cap != 0)
            .map(|edge| format!("x_{}_{}", edge.0, edge.1))
            .collect::<Vec<String>>()
            .join(" + ");

        let outgoing: String = graph
            .edges_directed(node, petgraph::Direction::Outgoing)
            .filter(|edge| edge.2.cap != 0)
            .map(|edge| format!("x_{}_{}", edge.0, edge.1))
            .collect::<Vec<String>>()
            .join(" + ");

        if incoming != "" && outgoing != "" {
            writeln!(writer, "s.t. node_{}: {} = {};", node, incoming, outgoing).unwrap();
        }
    }

    writeln!(writer, "solve;").unwrap();
    writeln!(writer, "display {};", objectives).unwrap();

    writeln!(writer, "end;").unwrap();
}

#[derive(Clone, Copy)]
struct Edge {
    cap: i32,
    flow: i32,
}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.flow, self.cap)
    }
}

fn print_flows(graph: &DiGraphMap<usize, Edge>) {
    println!("Flows:");
    for (node1, node2, edge) in graph.all_edges() {
        if edge.flow > 0 {
            println!("Edge from {} to {}: flow = {}", node1, node2, edge.flow);
        }
    }
    println!("-------------------------------------------------------------");
}

fn hamming_weight(x: usize) -> usize {
    x.count_ones() as usize
}

fn zero_count(x: usize, k: usize) -> usize {
    k - hamming_weight(x)
}

fn initialize_graph(k: usize) -> DiGraphMap<usize, Edge> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg} {elapsed_precise}")
            .unwrap(),
    );

    spinner.enable_steady_tick(Duration::from_millis(100));

    spinner.set_message("Initializing graph...");

    let mut graph = DiGraphMap::new();
    let n = 2usize.pow(k as u32);

    for i in 0..n {
        for j in 0..n {
            if i != j && (i ^ j).count_ones() == 1 && hamming_weight(i) < hamming_weight(j) {
                let l = *[
                    hamming_weight(i),
                    zero_count(i, k),
                    hamming_weight(j),
                    zero_count(j, k),
                ]
                .iter()
                .max()
                .unwrap();
                let cap = rand::thread_rng().gen_range(1..=2usize.pow(l as u32));
                graph.add_edge(
                    i,
                    j,
                    Edge {
                        cap: cap as i32,
                        flow: 0,
                    },
                );
                graph.add_edge(j, i, Edge { cap: 0, flow: 0 }); // Add reverse edge with capacity 0
            }
        }
    }

    spinner.finish_and_clear();

    graph
}

fn edmonds_karp(graph: &mut DiGraphMap<usize, Edge>, s: usize, t: usize) -> (i32, usize) {
    let mut flow = 0;
    let mut pred = HashMap::new();

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg} {elapsed_precise}")
            .unwrap(),
    );

    spinner.enable_steady_tick(Duration::from_millis(100));

    let mut augmenting_paths_count = 0;

    spinner.set_message(format!("iteration: {}", augmenting_paths_count));

    loop {
        pred.clear();

        debug!("cleared pred");

        let mut queue = Vec::new();
        queue.push(s);

        while let Some(node) = queue.pop() {
            if node == t {
                break;
            }
            debug!("  handling node: {}", node);
            for neighbor in graph.neighbors(node) {
                if !pred.contains_key(&neighbor) && neighbor != s {
                    let edge = graph.edge_weight(node, neighbor).unwrap().clone();
                    if edge.cap > edge.flow {
                        pred.insert(neighbor, (node, edge));
                        debug!("    inserted neighbor: {}, ({}, {})", neighbor, node, edge);
                        queue.push(neighbor);
                    }
                }
            }
        }

        debug!("bfs done");

        if !pred.contains_key(&t) {
            debug!("no path found");
            break;
        }

        augmenting_paths_count += 1;

        let mut df = i32::MAX;
        let mut node = t;
        debug!("finding df");
        while let Some(&(prev_node, edge)) = pred.get(&node) {
            df = cmp::min(df, edge.cap - edge.flow);
            debug!("  edge: {} -> {}, df: {}", prev_node, node, df);
            node = prev_node;
        }

        debug!("df: {}", df);

        debug!("updating flow");
        node = t;
        while let Some(&(prev_node, _)) = pred.get(&node) {
            debug!("  updating edge: {} -> {}", prev_node, node);
            let edge = graph.edge_weight_mut(prev_node, node).unwrap();
            edge.flow += df;
            let rev_edge = graph.edge_weight_mut(node, prev_node).unwrap();
            rev_edge.flow -= df;
            node = prev_node;
        }

        flow += df;

        spinner.set_message(format!("iteration: {}", augmenting_paths_count));

        debug!("new Flow: {}", flow)
    }

    spinner.finish_and_clear();

    (flow, augmenting_paths_count)
}

/// Implementation of the Edmonds-Karp algorithm for finding the maximum flow in a graph
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The size of the graph, between 1 and 16
    #[arg(long = "size")]
    k: usize,

    /// Whether to print the flow graph
    #[arg(long = "printFlow")]
    print_flow: bool,

    /// Whether to print glpk output
    #[arg(long = "glpk")]
    glpk: Option<PathBuf>,
}