use std::{collections::BTreeSet, str::FromStr};

use anyhow::{ensure, Context, Error, Result};

type Position = (isize, isize);

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn delta(self) -> Position {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        }
    }

    pub fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Clone, Debug)]
struct Problem {
    size: (isize, isize),
    blocks: BTreeSet<Position>,
    guard_pos: Position,
    guard_dir: Direction,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut blocks: BTreeSet<Position> = BTreeSet::new();
        let mut guard_pos: Option<Position> = None;
        let mut guard_dir: Option<Direction> = None;

        let lines: Vec<&str> = input.trim().lines().collect();
        let n = lines.len() as isize;
        let m = lines[0].len() as isize;

        for (i, line) in lines.iter().enumerate() {
            let chars: Vec<char> = line.chars().collect();
            ensure!(chars.len() == m as usize, "Invalid dimension");
            for (j, c) in chars.into_iter().enumerate() {
                if c == '#' {
                    blocks.insert((i as isize, j as isize));
                }
                if let Some(dir) = Direction::from_char(c) {
                    ensure!(guard_pos.is_none(), "Multiple guards");
                    ensure!(guard_dir.is_none(), "Multiple guards");
                    guard_pos = Some((i as isize, j as isize));
                    guard_dir = Some(dir);
                }
            }
        }

        let guard_pos = guard_pos.context("No guard")?;
        let guard_dir = guard_dir.context("No guard")?;

        Ok(Problem {
            size: (n, m),
            blocks,
            guard_pos,
            guard_dir,
        })
    }
}

fn solve(problem: &Problem) -> Result<usize> {
    let mut guard_pos = problem.guard_pos;
    let mut guard_dir = problem.guard_dir;
    let mut footprints: BTreeSet<Position> = BTreeSet::new();

    while guard_pos.0 >= 0
        && guard_pos.0 < problem.size.0
        && guard_pos.1 >= 0
        && guard_pos.1 < problem.size.1
    {
        footprints.insert(guard_pos);
        loop {
            let next_pos = (
                guard_pos.0 + guard_dir.delta().0,
                guard_pos.1 + guard_dir.delta().1,
            );
            if problem.blocks.contains(&next_pos) {
                guard_dir = guard_dir.turn_right();
            } else {
                guard_pos = next_pos;
                break;
            }
        }
    }

    Ok(footprints.len())
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
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 41);
        Ok(())
    }
}
