use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use log::debug;
use petgraph::graphmap::DiGraphMap;
use rand::Rng;
use std::cmp;
use std::collections::{HashMap, VecDeque};
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
        let file = File::create(path).unwrap();

        let mut writer = BufWriter::new(file);

        write_lp_model(&mut writer, &graph);
    }

    let algo = cli.algo;

    let now = std::time::Instant::now();
    let (flow, paths) = match algo.as_str() {
        "edmonds-karp" => edmonds_karp(&mut graph, 0, 2usize.pow(k as u32) - 1),
        "dinic" => dinic(&mut graph, 0, 2usize.pow(k as u32) - 1),
        _ => panic!("Unknown algorithm: {}", algo),
    };
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

fn write_lp_model(writer: &mut BufWriter<File>, graph: &DiGraphMap<usize, Edge>) {
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
        for shift in 0..k {
            let j = i ^ (1 << shift);
            if hamming_weight(i) < hamming_weight(j) {
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

fn dinic(graph: &mut DiGraphMap<usize, Edge>, s: usize, t: usize) -> (i32, usize) {
    let node_count = graph.node_count();
    let mut augmenting_paths_count = 0;
    let mut level = vec![-1; node_count];

    // Corner case
    if s == t {
        return (-1, augmenting_paths_count);
    }

    let mut max_flow = 0; // Initialize result

    // Augment the flow while there is path
    // from source to sink
    while bfs(graph, s, t, &mut level) == true {
        // store how many edges are visited
        // from V { 0 to V }
        let mut start = vec![0usize; node_count];

        // while flow is not zero in graph from S to D
        while let Some(flow) = send_flow(graph, s, t, i32::MAX, &mut start, &mut level) {
            // Add path flow to overall flow
            augmenting_paths_count += 1;
            max_flow += flow;
        }
    }

    // return maximum flow
    return (max_flow, augmenting_paths_count);
}

fn send_flow(
    graph: &mut DiGraphMap<usize, Edge>,
    u: usize,
    t: usize,
    flow: i32,
    start: &mut Vec<usize>,
    level: &mut Vec<i32>,
) -> Option<i32> {
    // Sink reached
    if u == t {
        return Some(flow);
    }

    // Traverse all adjacent edges one -by - one.
    while start[u] < graph.neighbors(u).count() {
        // Pick next edge from adjacency list of u
        let (_, v, e) = graph.edges(u).nth(start[u]).unwrap();

        if level[v] == level[u] + 1 && e.flow < e.cap {
            // find minimum flow from u to t
            let curr_flow = cmp::min(e.cap - e.flow, flow);

            if let Some(temp_flow) = send_flow(graph, v, t, curr_flow, start, level) {
                // add flow  to current edge
                let edge = graph.edge_weight_mut(u, v).unwrap();
                edge.flow += temp_flow;
                // subtract flow from reverse edge
                // of current edge
                let rev_edge = graph.edge_weight_mut(v, u).unwrap();
                rev_edge.flow -= temp_flow;

                return Some(temp_flow);
            }
        }
        start[u] += 1;
    }

    None
}

fn bfs(graph: &DiGraphMap<usize, Edge>, s: usize, t: usize, level: &mut Vec<i32>) -> bool {
    for i in 0..level.len() {
        level[i] = -1;
    }

    level[s] = 0; // Level of source vertex

    // Create a queue, enqueue source vertex
    // and mark source vertex as visited here
    // level[] array works as visited array also.
    let mut q = VecDeque::new();
    q.push_back(s);

    while let Some(u) = q.pop_front() {
        for i in graph.neighbors(u) {
            let e = graph.edge_weight(u, i).unwrap();
            if level[i] < 0 && e.flow < e.cap {
                level[i] = level[u] + 1;
                q.push_back(i);
            }
        }
    }

    // IF we can not reach to the sink we
    // return false else true
    if level[t] < 0 {
        false
    } else {
        true
    }
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

    /// which algorithm to use
    #[arg(long = "algo", default_value = "edmonds-karp")]
    algo: String,
}
