use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    ops::{Add, Mul},
    str::FromStr,
};

use anyhow::{bail, ensure, Error, Result};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point {
    pub i: usize,
    pub j: usize,
}

impl Point {
    pub const UP: Point = Point {
        i: -1isize as usize,
        j: 0,
    };
    pub const RIGHT: Point = Point { i: 0, j: 1 };
    pub const DOWN: Point = Point { i: 1, j: 0 };
    pub const LEFT: Point = Point {
        i: 0,
        j: -1isize as usize,
    };
    pub const DIRS: [Point; 4] = [Point::UP, Point::RIGHT, Point::DOWN, Point::LEFT];
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            i: self.i.wrapping_add(other.i),
            j: self.j.wrapping_add(other.j),
        }
    }
}

impl Mul<usize> for Point {
    type Output = Point;

    fn mul(self, m: usize) -> Point {
        Point {
            i: self.i.wrapping_mul(m),
            j: self.j.wrapping_mul(m),
        }
    }
}

#[derive(Clone, Debug)]
struct Problem {
    pub map: HashSet<Point>,
    pub start: Point,
    pub goal: Point,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut starts = Vec::new();
        let mut goals = Vec::new();
        let map: HashSet<Point> = input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(j, c)| {
                        let ok = match c {
                            'S' => {
                                starts.push(Point { i, j });
                                true
                            }
                            'E' => {
                                goals.push(Point { i, j });
                                true
                            }
                            '.' => true,
                            _ => false,
                        };
                        ok.then_some(Point { i, j })
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        ensure!(starts.len() == 1, "Invalid number of starts");
        ensure!(goals.len() == 1, "Invalid number of goals");
        let start = starts[0];
        let goal = goals[0];

        Ok(Problem { map, start, goal })
    }
}

fn solve(problem: &Problem) -> Result<usize> {
    let mut queue = BinaryHeap::new();
    let mut finalized = HashSet::new();
    let mut known_best: HashMap<Point, usize> = HashMap::new();

    for l in 0.. {
        let pos = problem.start + Point::RIGHT * l;
        if !problem.map.contains(&pos) {
            break;
        }
        let cost = l;
        queue.push(Reverse((cost, pos)));
        known_best.insert(pos, cost);
    }

    while let Some(Reverse((current_cost, current_pos))) = queue.pop() {
        if current_pos == problem.goal {
            assert_eq!(known_best.get(&current_pos), Some(&current_cost));
            return Ok(current_cost);
        }
        if !finalized.insert(current_pos) {
            continue;
        }

        for dir in Point::DIRS {
            for l in 0.. {
                let next_pos = current_pos + dir * l;
                let next_cost = current_cost + 1000 + l;
                if !problem.map.contains(&next_pos) {
                    break;
                }
                if let Some(&best) = known_best.get(&next_pos) {
                    if next_cost >= best {
                        continue;
                    }
                }
                queue.push(Reverse((next_cost, next_pos)));
                known_best.insert(next_pos, next_cost);
            }
        }
    }

    bail!("No path found")
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
    fn sample1() -> Result<()> {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 7036);
        Ok(())
    }

    #[test]
    fn sample2() -> Result<()> {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 11048);
        Ok(())
    }
}
