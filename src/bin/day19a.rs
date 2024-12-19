use std::str::FromStr;

use anyhow::{Context, Error, Result};
use itertools::Itertools;

#[derive(Clone, Debug)]
struct Problem {
    pub parts: Vec<Vec<char>>,
    pub towels: Vec<Vec<char>>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let (parts_str, towels_str) = input.split_once("\n\n").context("Invalid input")?;
        let parts = parts_str
            .split(", ")
            .map(|s| s.chars().collect_vec())
            .collect();
        let towels = towels_str
            .lines()
            .map(|s| s.chars().collect_vec())
            .collect();
        Ok(Problem { parts, towels })
    }
}

fn can_build(towel: &[char], parts: &[Vec<char>]) -> bool {
    let mut possible = vec![false; towel.len() + 1];
    possible[0] = true;
    for i in 0..towel.len() {
        if !possible[i] {
            continue;
        }
        for part in parts {
            let j = i + part.len();
            if j > towel.len() {
                continue;
            }
            if towel[i..j] == part[..] {
                possible[j] = true;
            }
        }
    }
    possible[towel.len()]
}

fn solve(problem: &Problem) -> Result<usize> {
    Ok(problem
        .towels
        .iter()
        .filter(|towel| can_build(towel, &problem.parts))
        .count())
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
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 6);
        Ok(())
    }
}
