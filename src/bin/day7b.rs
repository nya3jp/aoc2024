use std::str::FromStr;

use anyhow::{Context, Error, Result};

#[derive(Clone, Debug)]
struct Equation {
    target: u64,
    factors: Vec<u64>,
}

#[derive(Clone, Debug)]
struct Problem {
    equations: Vec<Equation>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let equations = input
            .trim()
            .lines()
            .map(|line| {
                let (target_str, factors_str) = line.split_once(": ").context("Invalid input")?;
                let target = target_str.parse()?;
                let factors = factors_str
                    .split_ascii_whitespace()
                    .map(|x| x.parse::<u64>())
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Equation { target, factors })
            })
            .collect::<Result<_>>()?;
        Ok(Problem { equations })
    }
}

fn concat_numbers(a: u64, b: u64) -> u64 {
    let mut m = 1;
    while b >= m {
        m *= 10;
    }
    a * m + b
}

fn search(current: u64, target: u64, factors: &[u64]) -> bool {
    if current > target {
        return false;
    }
    let Some((&first, new_factors)) = factors.split_first() else {
        return current == target;
    };
    search(current + first, target, new_factors)
        || search(current * first, target, new_factors)
        || search(concat_numbers(current, first), target, new_factors)
}

fn can_produce(equation: &Equation) -> bool {
    search(equation.factors[0], equation.target, &equation.factors[1..])
}

fn solve(problem: &Problem) -> Result<u64> {
    Ok(problem
        .equations
        .iter()
        .filter(|equation| can_produce(equation))
        .map(|equation| equation.target)
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
    fn sample() -> Result<()> {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 11387);
        Ok(())
    }
}
