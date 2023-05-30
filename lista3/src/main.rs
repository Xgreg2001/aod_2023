use crate::parsing::parse_ss;
use parsing::parse_dimacs_gr_to_petgraph;
use std::fs::read_to_string;
use std::path::PathBuf;

mod parsing;
mod algorithms;

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let data = read_to_string(args.gr_path).unwrap();

    let graph = parse_dimacs_gr_to_petgraph(data.as_str()).unwrap();

    let ss_contents = read_to_string(args.ss_path.unwrap()).unwrap();

    let (_, ss_config) = parse_ss(ss_contents.as_str()).unwrap();

    println!(
        "parsed graph with {} nodes and {} edges.",
        graph.node_count(),
        graph.edge_count()
    );
    println!("ss config: {:?}", ss_config)
}

#[derive(Debug)]
struct AppArgs {
    gr_path: PathBuf,
    ss_path: Option<PathBuf>,
    oss_path: Option<PathBuf>
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
