use std::{collections::BTreeSet, str::FromStr};

use anyhow::{ensure, Context, Error, Result};

type Position = (isize, isize);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct GuardState {
    pub pos: Position,
    pub dir: Direction,
}

#[derive(Clone, Debug)]
struct Problem {
    size: (isize, isize),
    blocks: BTreeSet<Position>,
    guard: GuardState,
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
            guard: GuardState {
                pos: guard_pos,
                dir: guard_dir,
            },
        })
    }
}

fn check_loop(problem: &Problem) -> Result<bool> {
    let mut state = problem.guard;
    let mut visited: BTreeSet<GuardState> = BTreeSet::new();

    while visited.insert(state) {
        if state.pos.0 < 0
            || state.pos.0 >= problem.size.0
            || state.pos.1 < 0
            || state.pos.1 >= problem.size.1
        {
            return Ok(false);
        }

        loop {
            let next_pos = (
                state.pos.0 + state.dir.delta().0,
                state.pos.1 + state.dir.delta().1,
            );
            if problem.blocks.contains(&next_pos) {
                state.dir = state.dir.turn_right();
            } else {
                state.pos = next_pos;
                break;
            }
        }
    }

    Ok(true)
}

fn solve(problem: &Problem) -> Result<usize> {
    let mut cnt = 0;
    for i in 0..problem.size.0 {
        for j in 0..problem.size.1 {
            let obstacle_pos = (i, j);
            if problem.blocks.contains(&obstacle_pos) {
                continue;
            }
            if problem.guard.pos == obstacle_pos {
                continue;
            }
            let mut new_blocks = problem.blocks.clone();
            new_blocks.insert(obstacle_pos);
            let new_problem = Problem {
                size: problem.size,
                blocks: new_blocks,
                guard: problem.guard,
            };
            if check_loop(&new_problem)? {
                cnt += 1;
            }
        }
    }

    Ok(cnt)
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
        assert_eq!(answer, 6);
        Ok(())
    }
}
