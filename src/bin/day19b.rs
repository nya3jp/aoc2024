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

fn count_ways(towel: &[char], parts: &[Vec<char>]) -> usize {
    let mut ways = vec![0; towel.len() + 1];
    ways[0] = 1;
    for i in 0..towel.len() {
        for part in parts {
            let j = i + part.len();
            if j > towel.len() {
                continue;
            }
            if towel[i..j] == part[..] {
                ways[j] += ways[i];
            }
        }
    }
    ways[towel.len()]
}

fn solve(problem: &Problem) -> Result<usize> {
    Ok(problem
        .towels
        .iter()
        .map(|towel| count_ways(towel, &problem.parts))
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
        assert_eq!(answer, 16);
        Ok(())
    }
}
