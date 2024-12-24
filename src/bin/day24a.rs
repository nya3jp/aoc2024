use std::{collections::HashMap, str::FromStr};

use anyhow::{bail, Context, Error, Result};
use itertools::Itertools;
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

fn fill_value(name: &str, gates: &HashMap<String, Gate>, values: &mut HashMap<String, u64>) -> u64 {
    if let Some(&value) = values.get(name) {
        return value;
    }
    let gate = gates.get(name).unwrap();
    let (input1, input2) = gate
        .inputs
        .iter()
        .map(|name| fill_value(name, gates, values))
        .collect_tuple()
        .unwrap();
    let value = gate.kind.apply(input1, input2);
    values.insert(name.to_string(), value);
    value
}

fn solve(problem: &Problem) -> Result<u64> {
    let mut values = problem.init_values.clone();
    let names: Vec<String> = problem
        .init_values
        .keys()
        .cloned()
        .chain(problem.gates.keys().cloned())
        .sorted()
        .collect();
    Ok(names
        .into_iter()
        .filter_map(|name| {
            if !name.starts_with('z') {
                return None;
            }
            let shift: usize = name[1..].parse().unwrap();
            let value = fill_value(&name, &problem.gates, &mut values);
            Some(value << shift)
        })
        .sum())
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
    fn sample1() -> Result<()> {
        let input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 4);
        Ok(())
    }

    #[test]
    fn sample2() -> Result<()> {
        let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 2024);
        Ok(())
    }
}
