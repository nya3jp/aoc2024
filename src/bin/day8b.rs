use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};

use anyhow::{ensure, Error, Result};

type Position = (usize, usize);

#[derive(Clone, Debug)]
struct Problem {
    bound: Position,
    antennas: BTreeMap<char, Vec<Position>>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let lines: Vec<&str> = input.lines().collect();
        ensure!(!lines.is_empty(), "No lines");

        let bound = (lines.len(), lines[0].len());
        let mut antennas: BTreeMap<char, Vec<Position>> = BTreeMap::new();
        for (i, line) in lines.iter().enumerate() {
            let chars: Vec<char> = line.chars().collect();
            ensure!(
                chars.len() == bound.1,
                "Mismatched line length at {} (got {}, want {})",
                i + 1,
                chars.len(),
                bound.1
            );
            for (j, &c) in chars.iter().enumerate() {
                if c != '.' {
                    antennas.entry(c).or_default().push((i, j));
                }
            }
        }

        Ok(Problem { bound, antennas })
    }
}

fn in_bound(p: Position, bound: Position) -> bool {
    p.0 < bound.0 && p.1 < bound.1
}

fn solve(problem: &Problem) -> Result<usize> {
    let mut antinodes: BTreeSet<Position> = BTreeSet::new();
    for same_antennas in problem.antennas.values() {
        for (i, &a) in same_antennas.iter().enumerate() {
            for (j, &b) in same_antennas.iter().enumerate() {
                if i == j {
                    continue;
                }
                let d = (b.0.wrapping_sub(a.0), b.1.wrapping_sub(a.1));
                let mut p = b;
                while in_bound(p, problem.bound) {
                    antinodes.insert(p);
                    p = (p.0.wrapping_add(d.0), p.1.wrapping_add(d.1));
                }
            }
        }
    }
    Ok(antinodes.len())
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
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 34);
        Ok(())
    }
}
