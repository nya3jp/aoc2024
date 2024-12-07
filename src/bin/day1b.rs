use std::{collections::HashMap, str::FromStr};

use anyhow::{Context, Error, Result};
use itertools::Itertools;

#[derive(Clone, Debug)]
struct Problem {
    lefts: Vec<isize>,
    rights: Vec<isize>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let both: Vec<(isize, isize)> = input
            .lines()
            .map(|line| -> Result<_> {
                let mut parts = line.split_whitespace();
                let left = parts.next().context("No left value")?;
                let right = parts.next().context("No right value")?;
                let left = left.parse::<isize>()?;
                let right = right.parse::<isize>()?;
                Ok((left, right))
            })
            .collect::<Result<_>>()?;
        let (lefts, rights) = both.into_iter().unzip();
        Ok(Problem { lefts, rights })
    }
}

fn solve(problem: &Problem) -> Result<isize> {
    let right_counts: HashMap<_, _> = problem
        .rights
        .iter()
        .copied()
        .sorted()
        .chunk_by(|r| *r)
        .into_iter()
        .map(|(r, rs)| (r, rs.count() as isize))
        .collect();
    let answer = problem
        .lefts
        .iter()
        .copied()
        .map(|l| l * right_counts.get(&l).unwrap_or(&0))
        .sum();
    Ok(answer)
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
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 31);
        Ok(())
    }
}
