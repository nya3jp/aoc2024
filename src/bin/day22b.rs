use std::{collections::HashMap, str::FromStr};

use anyhow::{Error, Result};
use itertools::Itertools;

#[derive(Clone, Debug)]
struct Problem {
    pub secrets: Vec<i64>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let secrets = input.lines().map(str::parse).collect::<Result<_, _>>()?;
        Ok(Problem { secrets })
    }
}

fn next(mut secret: i64) -> i64 {
    secret ^= secret << 6;
    secret &= 0xffffff;
    secret ^= secret >> 5;
    secret &= 0xffffff;
    secret ^= secret << 11;
    secret &= 0xffffff;
    secret
}

fn solve(problem: &Problem) -> Result<i64> {
    let buyers: Vec<Vec<i64>> = problem
        .secrets
        .iter()
        .copied()
        .map(|mut secret| {
            let mut prices = vec![secret % 10];
            for _ in 0..2000 {
                secret = next(secret);
                prices.push(secret % 10);
            }
            prices
        })
        .collect();

    let best = buyers
        .iter()
        .flat_map(|prices| -> HashMap<_, i64> {
            let mut entries = prices
                .iter()
                .copied()
                .tuple_windows()
                .map(|(a, b, c, d, e)| ((b - a, c - b, d - c, e - d), e))
                .collect_vec();
            entries.reverse();
            HashMap::from_iter(entries)
        })
        .sorted()
        .chunk_by(|(v, _)| *v)
        .into_iter()
        .map(|(_, prices)| prices.map(|(_, price)| price).sum())
        .max()
        .unwrap();

    Ok(best)
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
        let input = "1
2
3
2024
";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 23);
        Ok(())
    }
}
