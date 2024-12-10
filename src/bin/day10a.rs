use std::{
    collections::{HashMap, HashSet},
    ops::Add,
    str::FromStr,
};

use anyhow::{Context, Error, Result};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Position {
    pub i: i32,
    pub j: i32,
}

impl Position {
    const ADJACENTS: [Position; 4] = [
        Position { i: -1, j: 0 },
        Position { i: 0, j: 1 },
        Position { i: 1, j: 0 },
        Position { i: 0, j: -1 },
    ];
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            i: self.i + other.i,
            j: self.j + other.j,
        }
    }
}

#[derive(Clone, Debug)]
struct Problem {
    map: HashMap<Position, u32>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut map = HashMap::new();
        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                let h = c.to_digit(10).context("Not a digit")?;
                map.insert(
                    Position {
                        i: i as i32,
                        j: j as i32,
                    },
                    h,
                );
            }
        }
        Ok(Problem { map })
    }
}

fn solve(problem: &Problem) -> Result<usize> {
    let mut score = 0;

    let starts = problem
        .map
        .iter()
        .filter(|(_, &h)| h == 0)
        .map(|(pos, _)| pos);
    for &start in starts {
        let mut seen = HashSet::from([start]);
        let mut stack = vec![start];
        while let Some(current_pos) = stack.pop() {
            let current_height = *problem.map.get(&current_pos).unwrap();
            for &adj in &Position::ADJACENTS {
                let next_pos = current_pos + adj;
                if seen.contains(&next_pos) {
                    continue;
                }
                if let Some(&next_height) = problem.map.get(&next_pos) {
                    if next_height == current_height + 1 && seen.insert(next_pos) {
                        stack.push(next_pos);
                        if next_height == 9 {
                            score += 1;
                        }
                    }
                }
            }
        }
    }

    Ok(score)
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
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 36);
        Ok(())
    }
}
