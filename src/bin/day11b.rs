use std::{collections::BTreeMap, str::FromStr};

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

type Cache = BTreeMap<(u64, usize), usize>;

fn blink(n: u64, t: usize, cache: &mut Cache) -> usize {
    if t == 0 {
        return 1;
    }
    if let Some(&c) = cache.get(&(n, t)) {
        return c;
    }
    let c = {
        if n == 0 {
            blink(1, t - 1, cache)
        } else {
            let d = digits(n);
            if d % 2 == 0 {
                let half = 10u64.pow(d as u32 / 2);
                blink(n / half, t - 1, cache) + blink(n % half, t - 1, cache)
            } else {
                blink(n * 2024, t - 1, cache)
            }
        }
    };
    cache.insert((n, t), c);
    c
}

fn solve(problem: &Problem) -> Result<usize> {
    let mut cache = Cache::new();
    Ok(problem
        .stones
        .iter()
        .map(|&n| blink(n, 75, &mut cache))
        .sum())
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin().lock())?;
    let problem: Problem = input.trim().parse()?;
    let answer = solve(&problem)?;
    println!("{}", answer);
    Ok(())
}
