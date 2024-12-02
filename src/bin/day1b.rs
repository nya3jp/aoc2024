use std::collections::HashMap;

use anyhow::{Context, Result};
use itertools::Itertools;

fn parse_input(input: &str) -> Result<(Vec<isize>, Vec<isize>)> {
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
    Ok(both.into_iter().unzip())
}

fn solve(lefts: &[isize], rights: &[isize]) -> Result<isize> {
    let right_counts: HashMap<_, _> = rights
        .iter()
        .copied()
        .sorted()
        .chunk_by(|r| *r)
        .into_iter()
        .map(|(r, rs)| (r, rs.count() as isize))
        .collect();
    let answer = lefts
        .iter()
        .copied()
        .map(|l| l * right_counts.get(&l).unwrap_or(&0))
        .sum();
    Ok(answer)
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin().lock())?;
    let (lefts, rights) = parse_input(&input)?;
    let answer = solve(&lefts, &rights)?;
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
        let (lefts, rights) = parse_input(input)?;
        let answer = solve(&lefts, &rights)?;
        assert_eq!(answer, 31);
        Ok(())
    }
}
