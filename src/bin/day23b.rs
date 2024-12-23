use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::{Context, Error, Result};

type Node = String;

#[derive(Clone, Debug)]
struct Graph {
    pub edges: HashMap<Node, HashSet<Node>>,
}

#[derive(Clone, Debug)]
struct Problem {
    pub graph: Graph,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut edges: HashMap<Node, HashSet<Node>> = HashMap::new();
        for line in input.lines() {
            let (a, b) = line.split_once('-').context("Invalid edge")?;
            edges
                .entry(a.to_string())
                .or_default()
                .insert(b.to_string());
            edges
                .entry(b.to_string())
                .or_default()
                .insert(a.to_string());
        }
        Ok(Problem {
            graph: Graph { edges },
        })
    }
}

fn naive_search<'a>(
    chosen: &[&'a str],
    candidates: &HashSet<&'a str>,
    edges: &HashMap<&str, HashSet<&'a str>>,
    known_best: usize,
) -> Vec<&'a str> {
    if chosen.len() + candidates.len() <= known_best {
        return Vec::new();
    }
    if candidates.is_empty() {
        return chosen.to_vec();
    }

    let mut best = Vec::new();
    for &next in candidates {
        let new_chosen: Vec<&str> = chosen.iter().copied().chain([next]).collect();
        let new_candidates: HashSet<&str> =
            candidates.intersection(&edges[next]).copied().collect();
        let new_best = naive_search(
            &new_chosen,
            &new_candidates,
            edges,
            known_best.max(best.len()),
        );
        if new_best.len() > best.len() {
            best = new_best;
        }
    }
    best
}

fn solve(problem: &Problem) -> Result<String> {
    let edges = &problem.graph.edges;
    let edges: HashMap<&str, HashSet<&str>> = edges
        .iter()
        .map(|(a, bs)| (a.as_str(), bs.iter().map(|b| b.as_str()).collect()))
        .collect();

    let mut best = Vec::new();
    for (a, bs) in &edges {
        let new_best = naive_search(&[a], bs, &edges, best.len());
        if new_best.len() > best.len() {
            best = new_best;
        }
    }
    best.sort();
    Ok(best.join(","))
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin().lock())?;
    let problem: Problem = input.parse()?;
    let answer = solve(&problem)?;
    println!("{}", answer);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() -> Result<()> {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, "co,de,ka,ta");
        Ok(())
    }
}
