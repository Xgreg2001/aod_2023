use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use indicatif::ProgressBar;
use petgraph::graph::NodeIndex;

use parsing::parse_dimacs_gr_to_petgraph;

use crate::algorithms::{dijkstra_all, dijkstra_single};
use crate::parsing::parse_ss;

mod algorithms;
mod parsing;

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let gr_path = args.gr_path;
    let data = read_to_string(&gr_path).unwrap();
    let graph = parse_dimacs_gr_to_petgraph(data.as_str()).unwrap();

    if let Some(ss_path) = args.ss_path {
        let ss_contents = read_to_string(&ss_path).unwrap();

        let (_, ss_config) = parse_ss(ss_contents.as_str()).unwrap();

        let mut times: Vec<Duration> = Vec::with_capacity(ss_config.num_sources);

        let min_cost = graph.edge_weights().min().unwrap();
        let max_cost = graph.edge_weights().max().unwrap();

        let bar = ProgressBar::new(ss_config.sources.len() as u64);

        for source in ss_config.sources {
            let start_node = NodeIndex::new(source);
            let now = Instant::now();
            dijkstra_all(&graph, start_node);
            let elapsed = now.elapsed();
            times.push(elapsed);

            bar.inc(1);
        }

        bar.finish();

        let count: f64 = times.len() as f64;
        let mean_time_millis: f64 = times.iter().map(|d| d.as_millis() as f64).sum::<f64>() / count;

        if let Some(oss_path) = args.oss_path {
            let mut result_file = File::create(oss_path).unwrap();

            writeln!(result_file, "f {} {}", gr_path.display(), ss_path.display()).unwrap();
            writeln!(
                result_file,
                "g {} {} {} {}",
                graph.node_count(),
                graph.edge_count(),
                min_cost,
                max_cost
            )
            .unwrap();

            writeln!(result_file, "t {}", mean_time_millis).unwrap();
        } else {
            println!("f {} {}", gr_path.display(), ss_path.display());
            println!(
                "g {} {} {} {}",
                graph.node_count(),
                graph.edge_count(),
                min_cost,
                max_cost
            );
            println!("t {}", mean_time_millis);
        }
    } else if let Some(p2p_path) = args.p2p_path {
        let p2p_contents = read_to_string(&p2p_path).unwrap();

        let (_, p2p_config) = parsing::parse_p2p(p2p_contents.as_str()).unwrap();

        let min_cost = graph.edge_weights().min().unwrap();
        let max_cost = graph.edge_weights().max().unwrap();

        let bar = ProgressBar::new(p2p_config.pairs.len() as u64);

        let mut distances = Vec::with_capacity(p2p_config.pairs.len());

        for pair in &p2p_config.pairs {
            let start_node = NodeIndex::new(pair.0);
            let end_node = NodeIndex::new(pair.1);
            distances.push(dijkstra_single(&graph, start_node, end_node));
            bar.inc(1);
        }

        bar.finish();

        if let Some(op2p_path) = args.op2p_path {
            let mut result_file = File::create(op2p_path).unwrap();

            writeln!(
                result_file,
                "f {} {}",
                gr_path.display(),
                p2p_path.display()
            )
            .unwrap();
            writeln!(
                result_file,
                "g {} {} {} {}",
                graph.node_count(),
                graph.edge_count(),
                min_cost,
                max_cost
            )
            .unwrap();

            for (pair, distance) in p2p_config.pairs.iter().zip(&distances) {
                writeln!(result_file, "d {} {} {}", pair.0 + 1, pair.1 + 1, distance).unwrap();
            }
        } else {
            println!("f {} {}", gr_path.display(), p2p_path.display());
            println!(
                "g {} {} {} {}",
                graph.node_count(),
                graph.edge_count(),
                min_cost,
                max_cost
            );

            for (pair, distance) in p2p_config.pairs.iter().zip(&distances) {
                println!("d {} {} {}", pair.0 + 1, pair.1 + 1, distance);
            }
        }
    }
}

#[derive(Debug)]
struct AppArgs {
    gr_path: PathBuf,
    ss_path: Option<PathBuf>,
    oss_path: Option<PathBuf>,
    p2p_path: Option<PathBuf>,
    op2p_path: Option<PathBuf>,
}

const HELP: &str = "\
Lista 3

USAGE:
  lista3 -d GR_PATH [OPTIONS]

FLAGS:
  -h, --help        Prints help information

OPTIONS:
  -ss SS_PATH       Path to .ss file
  -oss OSS_PATH     Path to output file
  -p2p P2P_PATH     Path to .p2p file
  -op2p OP2P_PATH   Path to output file
  -d GR_PATH        Path to .gr file

";

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let args = AppArgs {
        gr_path: pargs.value_from_os_str("-d", parse_path)?,
        ss_path: pargs.opt_value_from_os_str("-ss", parse_path)?,
        oss_path: pargs.opt_value_from_os_str("-oss", parse_path)?,
        p2p_path: pargs.opt_value_from_os_str("-p2p", parse_path)?,
        op2p_path: pargs.opt_value_from_os_str("-op2p", parse_path)?,
    };

    // It's up to the caller what to do with the remaining arguments.
    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}

fn parse_path(s: &std::ffi::OsStr) -> Result<PathBuf, &'static str> {
    Ok(s.into())
}
