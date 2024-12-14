use std::str::FromStr;

use anyhow::{Context, Error, Result};
use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug)]
struct Machine {
    pub a: Point,
    pub b: Point,
    pub t: Point,
}

impl FromStr for Machine {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let pattern = Regex::new(
            r"^Button A: X\+([0-9]+), Y\+([0-9]+)\nButton B: X\+([0-9]+), Y\+([0-9]+)\nPrize: X=([0-9]+), Y=([0-9]+)$",
        )?;
        let c = pattern.captures(input).context("Regex mismatch")?;
        let ax: i32 = c.get(1).unwrap().as_str().parse()?;
        let ay: i32 = c.get(2).unwrap().as_str().parse()?;
        let bx: i32 = c.get(3).unwrap().as_str().parse()?;
        let by: i32 = c.get(4).unwrap().as_str().parse()?;
        let tx: i32 = c.get(5).unwrap().as_str().parse()?;
        let ty: i32 = c.get(6).unwrap().as_str().parse()?;
        Ok(Machine {
            a: Point { x: ax, y: ay },
            b: Point { x: bx, y: by },
            t: Point { x: tx, y: ty },
        })
    }
}

#[derive(Clone, Debug)]
struct Problem {
    machines: Vec<Machine>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let machines = input
            .trim()
            .split("\n\n")
            .map(|s| s.parse::<Machine>())
            .collect::<Result<_>>()?;
        Ok(Problem { machines })
    }
}

fn solve_machine(machine: &Machine) -> Option<i32> {
    (0..=100)
        .cartesian_product(0..=100)
        .filter_map(|(a, b)| {
            let x = a * machine.a.x + b * machine.b.x;
            let y = a * machine.a.y + b * machine.b.y;
            if x == machine.t.x && y == machine.t.y {
                Some(a * 3 + b)
            } else {
                None
            }
        })
        .min()
}

fn solve(problem: &Problem) -> Result<i32> {
    let tokens = problem.machines.iter().filter_map(solve_machine).sum();
    Ok(tokens)
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin().lock())?;
    let problem: Problem = input.trim().parse()?;
    let answer = solve(&problem)?;
    println!("{}", answer);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() -> Result<()> {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 480);
        Ok(())
    }
}
