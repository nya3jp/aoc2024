use std::{
    collections::{HashSet, VecDeque},
    ops::Add,
    str::FromStr,
};

use anyhow::{Context, Error, Result};
use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub const RIGHT: Point = Point { x: 1, y: 0 };
    pub const DOWN: Point = Point { x: 0, y: 1 };
    pub const LEFT: Point = Point { x: -1, y: 0 };
    pub const UP: Point = Point { x: 0, y: -1 };
    pub const DIRS: [Point; 4] = [Point::RIGHT, Point::DOWN, Point::LEFT, Point::UP];

    pub fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let (x_str, y_str) = input.split_once(',').context("Invalid coordinates")?;
        let x = x_str.parse()?;
        let y = y_str.parse()?;
        Ok(Point { x, y })
    }
}

#[derive(Clone, Debug)]
struct Problem {
    pub schedule: Vec<Point>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let schedule = input.lines().map(str::parse).collect::<Result<_>>()?;
        Ok(Problem { schedule })
    }
}

fn find_shortest_path(map: &HashSet<Point>, start: Point, goal: Point) -> Option<usize> {
    let mut queue: VecDeque<(Point, usize)> = VecDeque::from([(start, 0)]);
    let mut visited: HashSet<Point> = HashSet::from([start]);

    while let Some((current_pos, current_dist)) = queue.pop_front() {
        if current_pos == goal {
            return Some(current_dist);
        }

        for dir in Point::DIRS {
            let next_pos = current_pos + dir;
            if map.contains(&next_pos) && visited.insert(next_pos) {
                queue.push_back((next_pos, current_dist + 1));
            }
        }
    }

    None
}

fn solve(problem: &Problem, size: isize, limit: usize) -> Result<usize> {
    let mut map: HashSet<Point> = (0..size)
        .cartesian_product(0..size)
        .map(|(x, y)| Point::new(x, y))
        .collect();
    for p in &problem.schedule[0..limit] {
        map.remove(p);
    }
    find_shortest_path(&map, Point::new(0, 0), Point::new(size - 1, size - 1))
        .context("No path found")
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin().lock())?;
    let problem: Problem = input.parse()?;
    let answer = solve(&problem, 71, 1024)?;
    println!("{}", answer);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() -> Result<()> {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem, 7, 12)?;
        assert_eq!(answer, 22);
        Ok(())
    }
}
