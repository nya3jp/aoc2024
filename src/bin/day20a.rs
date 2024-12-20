use std::{
    collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
    ops::Add,
    str::FromStr,
};

use anyhow::{bail, ensure, Context, Error, Result};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Point {
    pub i: isize,
    pub j: isize,
}

impl Point {
    pub const RIGHT: Point = Point { i: 0, j: 1 };
    pub const DOWN: Point = Point { i: 1, j: 0 };
    pub const LEFT: Point = Point { i: 0, j: -1 };
    pub const UP: Point = Point { i: -1, j: 0 };
    pub const DIRS: [Point; 4] = [Point::RIGHT, Point::DOWN, Point::LEFT, Point::UP];

    pub fn new(i: isize, j: isize) -> Self {
        Point { i, j }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            i: self.i + other.i,
            j: self.j + other.j,
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
        let mut map: HashSet<Point> = HashSet::new();
        let mut starts: Vec<Point> = Vec::new();
        let mut goals: Vec<Point> = Vec::new();
        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                let p = Point::new(i as isize, j as isize);
                match c {
                    '.' => {
                        map.insert(p);
                    }
                    'S' => {
                        starts.push(p);
                        map.insert(p);
                    }
                    'E' => {
                        goals.push(p);
                        map.insert(p);
                    }
                    '#' => {}
                    _ => bail!("Invalid character: {}", c),
                }
            }
        }

        ensure!(starts.len() == 1, "Expected exactly one start");
        ensure!(goals.len() == 1, "Expected exactly one goal");

        let start = starts[0];
        let goal = goals[0];

        Ok(Problem { map, start, goal })
    }
}

fn bfs(map: &HashSet<Point>, start: Point) -> HashMap<Point, usize> {
    let mut queue = VecDeque::from([(start, 0)]);
    let mut distances = HashMap::from([(start, 0)]);

    while let Some((current_pos, current_dist)) = queue.pop_front() {
        for dir in Point::DIRS {
            let next_pos = current_pos + dir;
            let next_dist = current_dist + 1;
            if !map.contains(&next_pos) {
                continue;
            }
            if let Entry::Vacant(entry) = distances.entry(next_pos) {
                entry.insert(next_dist);
                queue.push_back((next_pos, next_dist));
            }
        }
    }

    distances
}

fn solve(problem: &Problem, min_save: usize) -> Result<usize> {
    let from_start = bfs(&problem.map, problem.start);
    let from_goal = bfs(&problem.map, problem.goal);

    let mut count = 0;
    let original_distance = *from_start.get(&problem.goal).context("No path found")?;
    for &cheat_start in from_start.keys() {
        for dir1 in Point::DIRS {
            let cheat_middle = cheat_start + dir1;
            if from_start.contains_key(&cheat_middle) {
                continue;
            }
            for dir2 in Point::DIRS {
                let cheat_end = cheat_middle + dir2;
                if !from_start.contains_key(&cheat_end) {
                    continue;
                }
                let cheat_distance = from_start[&cheat_start] + 2 + from_goal[&cheat_end];
                if cheat_distance < original_distance {
                    let save = original_distance - cheat_distance;
                    if save >= min_save {
                        count += 1;
                    }
                }
            }
        }
    }

    Ok(count)
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin().lock())?;
    let problem: Problem = input.parse()?;
    let answer = solve(&problem, 100)?;
    println!("{}", answer);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() -> Result<()> {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem, 1)?;
        assert_eq!(answer, 44);
        Ok(())
    }
}
