use std::str::FromStr;

use anyhow::{Context, Error, Result};

#[derive(Clone, Debug)]
struct Problem {
    pub locks: Vec<Vec<usize>>,
    pub keys: Vec<Vec<usize>>,
    pub n: usize,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut locks: Vec<Vec<usize>> = Vec::new();
        let mut keys: Vec<Vec<usize>> = Vec::new();
        for block in input.split("\n\n") {
            let mut pattern: Vec<Option<usize>> = Vec::new();
            let lines: Vec<&str> = block.trim().lines().collect();
            let n = lines.len();
            let is_lock = block.starts_with('#');
            if is_lock {
                for (i, line) in lines.into_iter().enumerate() {
                    pattern.resize(line.chars().count(), None);
                    for (j, c) in line.chars().enumerate() {
                        if c == '.' && pattern[j].is_none() {
                            pattern[j] = Some(i);
                        }
                    }
                }
            } else {
                for (i, line) in lines.into_iter().enumerate().rev() {
                    pattern.resize(line.chars().count(), None);
                    for (j, c) in line.chars().enumerate() {
                        if c == '.' && pattern[j].is_none() {
                            pattern[j] = Some(n - 1 - i);
                        }
                    }
                }
            }
            let pattern: Vec<usize> = pattern
                .into_iter()
                .collect::<Option<_>>()
                .context("Invalid pattern")?;
            if is_lock {
                locks.push(pattern);
            } else {
                keys.push(pattern);
            }
        }
        let n = input.split("\n\n").next().context("Empty input")?.lines().count();
        Ok(Problem { locks, keys, n })
    }
}

fn solve(problem: &Problem) -> Result<usize> {
    let mut count = 0;
    for lock in &problem.locks {
        for key in &problem.keys {
            if lock.iter().zip(key.iter()).map(|(a, b)| a+b).max().unwrap() <= problem.n {
                count += 1;
            }
        }
    }
    Ok(count)
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
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 3);
        Ok(())
    }
}
