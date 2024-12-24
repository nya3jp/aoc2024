use std::{collections::HashMap, str::FromStr};

use anyhow::{bail, Context, Error, Result};
use regex::Regex;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GateKind {
    And,
    Or,
    Xor,
}

impl GateKind {
    fn apply(self, input1: u64, input2: u64) -> u64 {
        match self {
            GateKind::And => input1 & input2,
            GateKind::Or => input1 | input2,
            GateKind::Xor => input1 ^ input2,
        }
    }
}

impl std::fmt::Display for GateKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GateKind::And => write!(f, "AND"),
            GateKind::Or => write!(f, "OR"),
            GateKind::Xor => write!(f, "XOR"),
        }
    }
}

impl FromStr for GateKind {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "AND" => Ok(GateKind::And),
            "OR" => Ok(GateKind::Or),
            "XOR" => Ok(GateKind::Xor),
            _ => bail!("Invalid gate kind: {}", s),
        }
    }
}

#[derive(Clone, Debug)]
struct Gate {
    pub kind: GateKind,
    pub inputs: [String; 2],
}

#[derive(Clone, Debug)]
struct Problem {
    pub init_values: HashMap<String, u64>,
    pub gates: HashMap<String, Gate>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let (init_values_str, gates_str) = input.split_once("\n\n").context("Invalid input")?;

        let init_values: HashMap<String, u64> = init_values_str
            .lines()
            .map(|line| {
                let pattern = Regex::new("^([a-z0-9]{3}): ([01])$").unwrap();
                let captures = pattern.captures(line).context("Invalid initial value")?;
                let name = captures[1].to_string();
                let value = captures[2].parse()?;
                Ok((name, value))
            })
            .collect::<Result<_>>()?;

        let gates: HashMap<String, Gate> = gates_str
            .lines()
            .map(|line| {
                let pattern =
                    Regex::new("^([a-z0-9]{3}) (AND|OR|XOR) ([a-z0-9]{3}) -> ([a-z0-9]{3})$")
                        .unwrap();
                let captures = pattern.captures(line).context("Invalid gate definition")?;
                let inputs = [captures[1].to_string(), captures[3].to_string()];
                let output = captures[4].to_string();
                let kind: GateKind = captures[2].parse()?;
                Ok((output, Gate { kind, inputs }))
            })
            .collect::<Result<_>>()?;

        Ok(Problem { init_values, gates })
    }
}

fn solve(problem: &Problem) -> Result<String> {
    println!("digraph G {{");
    for name in problem.init_values.keys() {
        println!("  {};", name);
    }
    for (output, gate) in &problem.gates {
        println!("  {};", output);
        println!("  {}_op [shape = box, label = \"{}\"];", output, gate.kind);
        println!("  {}_op -> {};", output, output);
        for input in &gate.inputs {
            println!("  {} -> {}_op;", input, output);
        }
    }
    println!("}}");

    Ok(String::new())
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin().lock())?;
    let problem: Problem = input.parse()?;
    let answer = solve(&problem)?;
    println!("{}", answer);
    Ok(())
}
