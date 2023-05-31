use std::collections::HashMap;

use nom::{
    bytes::complete::tag, character::complete::digit1, character::complete::line_ending,
    character::complete::multispace1, combinator::map_res, combinator::opt, IResult,
    sequence::tuple,
};
use petgraph::Graph;
use petgraph::Undirected;

#[derive(Debug)]
struct GraphEdge {
    source: usize,
    target: usize,
    weight: u64,
}

#[derive(Debug)]
pub struct ProblemSpecSS {
    pub num_sources: usize,
    pub sources: Vec<usize>,
}

pub struct ProblemSpecP2P {
    pub num_pairs: usize,
    pub pairs: Vec<(usize, usize)>,
}

fn parse_comment(input: &str) -> IResult<&str, ()> {
    let (input, _) = tuple((
        tag("c"),
        opt(nom::bytes::complete::is_not("\n\r")),
        line_ending,
    ))(input)?;
    Ok((input, ()))
}

fn parse_problem_gr(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, (_, num_nodes, _, num_edges, _)) = tuple((
        tag("p sp "),
        map_res(digit1, str::parse::<usize>),
        multispace1,
        map_res(digit1, str::parse::<usize>),
        line_ending,
    ))(input)?;
    Ok((input, (num_nodes, num_edges)))
}

fn parse_problem_ss(input: &str) -> IResult<&str, usize> {
    let (input, (_, num_sources, _)) = tuple((
        tag("p aux sp ss "),
        map_res(digit1, str::parse::<usize>),
        line_ending,
    ))(input)?;
    Ok((input, num_sources))
}

fn parse_source(input: &str) -> IResult<&str, usize> {
    let (input, (_, source, _)) =
        tuple((tag("s "), map_res(digit1, str::parse::<usize>), line_ending))(input)?;
    Ok((input, source))
}

fn parse_edge(input: &str) -> IResult<&str, GraphEdge> {
    let (input, (_, _, source, _, target, _, weight, _)) = tuple((
        tag("a"),
        multispace1,
        map_res(digit1, str::parse::<usize>),
        multispace1,
        map_res(digit1, str::parse::<usize>),
        multispace1,
        map_res(digit1, str::parse::<u64>),
        line_ending,
    ))(input)?;
    Ok((
        input,
        GraphEdge {
            source,
            target,
            weight,
        },
    ))
}

fn parse_dimacs_gr(input: &str) -> IResult<&str, (usize, usize, Vec<GraphEdge>)> {
    let mut remaining_input = input;
    // remove all comments from the be of the file
    while let Ok((input, Some(_))) = opt(parse_comment)(remaining_input) {
        remaining_input = input;
    }

    let (mut remaining_input, (num_nodes, num_edges)) = parse_problem_gr(remaining_input)?;
    let mut edges = Vec::with_capacity(num_edges);

    loop {
        if let Ok((input, Some(_))) = opt(parse_comment)(remaining_input) {
            remaining_input = input;
            continue;
        } else {
            match parse_edge(remaining_input) {
                Ok((input, edge)) => {
                    remaining_input = input;
                    edges.push(edge);
                }
                Err(_) => {
                    break;
                }
            }
        }
    }

    Ok((remaining_input, (num_nodes, num_edges, edges)))
}

pub fn parse_ss(input: &str) -> IResult<&str, ProblemSpecSS> {
    let mut remaining_input = input;
    // remove all comments from the be of the file
    while let Ok((input, Some(_))) = opt(parse_comment)(remaining_input) {
        remaining_input = input;
    }

    let (mut remaining_input, num_sources) = parse_problem_ss(remaining_input)?;

    let mut sources = Vec::with_capacity(num_sources);

    loop {
        if let Ok((input, Some(_))) = opt(parse_comment)(remaining_input) {
            remaining_input = input;
            continue;
        } else {
            match parse_source(remaining_input) {
                Ok((input, source)) => {
                    remaining_input = input;
                    sources.push(source - 1);
                }
                Err(_) => {
                    break;
                }
            }
        }
    }

    Ok((
        remaining_input,
        ProblemSpecSS {
            num_sources,
            sources,
        },
    ))
}

pub fn parse_p2p(input: &str) -> IResult<&str, ProblemSpecP2P> {
    let mut remaining_input = input;
    // remove all comments from the be of the file
    while let Ok((input, Some(_))) = opt(parse_comment)(remaining_input) {
        remaining_input = input;
    }

    let (mut remaining_input, num_pairs) = parse_problem_p2p(remaining_input)?;

    let mut pairs = Vec::with_capacity(num_pairs);

    loop {
        if let Ok((input, Some(_))) = opt(parse_comment)(remaining_input) {
            remaining_input = input;
            continue;
        } else {
            match parse_pair(remaining_input) {
                Ok((input, pair)) => {
                    remaining_input = input;
                    pairs.push(pair);
                }
                Err(_) => {
                    break;
                }
            }
        }
    }

    Ok((remaining_input, ProblemSpecP2P { num_pairs, pairs }))
}

fn parse_pair(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, (_, _, source, _, target, _)) = tuple((
        tag("q"),
        multispace1,
        map_res(digit1, str::parse::<usize>),
        multispace1,
        map_res(digit1, str::parse::<usize>),
        line_ending,
    ))(input)?;
    Ok((
        input,
        (
            source - 1,
            target - 1,
        ),
    ))
}

fn parse_problem_p2p(input: &str) -> IResult<&str, usize> {
    let (input, (_, num_sources, _)) = tuple((
        tag("p aux sp p2p "),
        map_res(digit1, str::parse::<usize>),
        line_ending,
    ))(input)?;
    Ok((input, num_sources))
}

pub fn parse_dimacs_gr_to_petgraph(input: &str) -> Result<Graph<(), u64, Undirected>, String> {
    match parse_dimacs_gr(input) {
        Ok((_, (num_nodes, num_edges, edges))) => {
            let mut graph = Graph::<(), u64, Undirected>::with_capacity(num_nodes, num_edges);
            let mut nodes = HashMap::with_capacity(num_nodes);
            for edge in edges {
                let source_node = *nodes
                    .entry(edge.source)
                    .or_insert_with(|| graph.add_node(()));
                let target_node = *nodes
                    .entry(edge.target)
                    .or_insert_with(|| graph.add_node(()));
                graph.add_edge(source_node, target_node, edge.weight);
            }
            Ok(graph)
        }

        Err(e) => Err(format!("Parsing error: {:?}", e)),
    }
}
