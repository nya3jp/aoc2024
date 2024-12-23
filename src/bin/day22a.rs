use std::str::FromStr;

use anyhow::{Error, Result};

#[derive(Clone, Debug)]
struct Problem {
    pub secrets: Vec<u64>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let secrets = input.lines().map(str::parse).collect::<Result<_, _>>()?;
        Ok(Problem { secrets })
    }
}

fn next(mut secret: u64) -> u64 {
    secret ^= secret << 6;
    secret &= 0xffffff;
    secret ^= secret >> 5;
    secret &= 0xffffff;
    secret ^= secret << 11;
    secret &= 0xffffff;
    secret
}

fn solve(problem: &Problem) -> Result<u64> {
    Ok(problem
        .secrets
        .iter()
        .copied()
        .map(|mut secret| {
            for _ in 0..2000 {
                secret = next(secret);
            }
            secret
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
    use itertools::Itertools;

    use super::*;

    #[test]
    fn sample() -> Result<()> {
        let input = "1
10
100
2024
";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 37327623);
        Ok(())
    }

    #[test]
    fn next_example() -> Result<()> {
        const EXAMPLE: [u64; 11] = [
            123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484,
            7753432, 5908254,
        ];
        for (&a, &b) in EXAMPLE.iter().tuple_windows() {
            assert_eq!(next(a), b);
        }
        Ok(())
    }
}
