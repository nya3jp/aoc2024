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

fn solve(problem: &Problem) -> Result<usize> {
    let mut count = 0;
    let edges = &problem.graph.edges;
    for (a, bs) in edges {
        for b in bs {
            if b <= a {
                continue;
            }
            for c in edges.get(b).unwrap() {
                if c <= b {
                    continue;
                }
                if !bs.contains(c) {
                    continue;
                }
                if a.starts_with('t') || b.starts_with('t') || c.starts_with('t') {
                    count += 1;
                }
            }
        }
    }
    Ok(count)
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
        assert_eq!(answer, 7);
        Ok(())
    }
}
