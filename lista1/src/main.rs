use std::env;

use lista1::*;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        println!("Usage: {} <mode> <file_path>", args[0]);
        return;
    }

    let mode = &args[1];
    let file_path = &args[2];

    let graph_result = Graph::build_from_file(file_path);

    let mut graph = match graph_result {
        Ok(graph) => {
            println!(
                "Graph of size {} built from file: {}",
                graph.get_n(),
                file_path
            );
            graph
        }
        Err(e) => {
            println!("ERROR: {}", e);
            return;
        }
    };

    match mode.as_str() {
        "dfs" => {
            let mut f = |node: &Node<i32>| {
                println!("Node: {}", node.index + 1);
            };
            graph.dfs(&mut f);
        }
        "bfs" => {
            let mut f = |node: &Node<i32>| {
                println!("Node: {}", node.index + 1);
            };
            graph.bfs(&mut f);
        }
        "topological" => {
            let ordering = graph.topological_sort();

            match ordering {
                Some(ordering) => {
                    if graph.get_n() <= 200 {
                        println!("Topological ordering:");
                        for i in ordering {
                            println!("{}", i);
                        }
                    }
                    println!("Graph is a DAG");
                }
                None => {
                    println!("Graph is not a DAG");
                }
            }
        }
        "components" => {
            let components = graph.find_strongly_connected_components();
            println!(
                "Number of strongly connected components: {}",
                components.len()
            );

            println!("Strongly connected components:");
            if graph.get_n() > 200 {
                for component in components {
                    println!("Component of size: {}", component.len());
                }
            } else {
                for component in components {
                    for node in component {
                        print!("{} ", node + 1);
                    }
                    println!();
                }
            }
        }
        "bipartite" => {
            if let Some((left, right)) = graph.get_bipartition() {
                println!("Graph is bipartite");
                if graph.get_n() <= 200 {
                    println!("Left component:");
                    for node in left {
                        print!("{} ", node + 1);
                    }
                    println!();
                    println!("Right component:");
                    for node in right {
                        print!("{} ", node + 1);
                    }
                    println!();
                }
            } else {
                println!("Graph is not bipartite");
            }
        }
        _ => {
            println!("Unknown mode: {}", mode);
        }
    }
}
