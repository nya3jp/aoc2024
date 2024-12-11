use std::str::FromStr;

use anyhow::{Error, Result};

#[derive(Clone, Debug)]
struct Problem {
    stones: Vec<u64>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let stones: Vec<u64> = input
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;
        Ok(Problem { stones })
    }
}

fn digits(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    for p in 1.. {
        if n < 10u64.pow(p) {
            return p as u64;
        }
    }
    unreachable!();
}

fn blink_once(stones: &[u64]) -> Vec<u64> {
    stones
        .iter()
        .flat_map(|&n| {
            if n == 0 {
                return vec![1];
            }
            let d = digits(n);
            if d % 2 == 0 {
                let half = 10u64.pow(d as u32 / 2);
                return vec![n / half, n % half];
            }
            vec![n * 2024]
        })
        .collect()
}

fn solve(problem: &Problem) -> Result<usize> {
    let mut stones = problem.stones.clone();
    for _ in 0..25 {
        stones = blink_once(&stones);
    }
    Ok(stones.len())
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
        let input = "125 17";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 55312);
        Ok(())
    }
}
