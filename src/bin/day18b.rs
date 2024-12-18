use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    ops::Add,
    str::FromStr,
};

use anyhow::{ensure, Context, Error, Result};
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

#[derive(Clone, Copy, Debug)]
enum UnionFindEntry<T> {
    Root { size: usize },
    Child { parent: T },
}

#[derive(Clone, Debug)]
struct UnionFind<T> {
    entries: HashMap<T, UnionFindEntry<T>>,
}

impl<T> UnionFind<T> {
    pub fn new() -> Self {
        UnionFind {
            entries: HashMap::new(),
        }
    }
}

impl<T: Eq + PartialEq + Hash> UnionFind<T> {
    pub fn add(&mut self, item: T) {
        self.entries
            .entry(item)
            .or_insert(UnionFindEntry::Root { size: 1 });
    }
}

impl<T: Copy + Eq + PartialEq + Hash> UnionFind<T> {
    pub fn find(&mut self, item: T) -> Option<(T, usize)> {
        match self.entries.get(&item) {
            Some(UnionFindEntry::Root { size }) => Some((item, *size)),
            Some(UnionFindEntry::Child { parent }) => {
                let (root, size) = self.find(*parent)?;
                self.entries
                    .insert(item, UnionFindEntry::Child { parent: root });
                Some((root, size))
            }
            None => None,
        }
    }
}

impl<T: Copy + Eq + PartialEq + Hash> UnionFind<T> {
    pub fn merge(&mut self, a: T, b: T) -> Option<(T, usize)> {
        let (a_root, a_size) = self.find(a)?;
        let (b_root, b_size) = self.find(b)?;

        if a_root == b_root {
            Some((a_root, a_size))
        } else if a_size < b_size {
            self.entries
                .insert(a_root, UnionFindEntry::Child { parent: b_root });
            self.entries.insert(
                b_root,
                UnionFindEntry::Root {
                    size: a_size + b_size,
                },
            );
            Some((b_root, a_size + b_size))
        } else {
            self.entries
                .insert(b_root, UnionFindEntry::Child { parent: a_root });
            self.entries.insert(
                a_root,
                UnionFindEntry::Root {
                    size: a_size + b_size,
                },
            );
            Some((a_root, a_size + b_size))
        }
    }
}

impl<T: Copy + Eq + PartialEq + Hash> UnionFind<T> {
    pub fn same(&mut self, a: T, b: T) -> Option<bool> {
        let (a_root, _) = self.find(a)?;
        let (b_root, _) = self.find(b)?;
        Some(a_root == b_root)
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

fn solve(problem: &Problem, size: isize) -> Result<Point> {
    let mut map: HashSet<Point> = (0..size)
        .cartesian_product(0..size)
        .map(|(x, y)| Point::new(x, y))
        .collect();
    let mut uf = UnionFind::new();
    for &p in &map {
        uf.add(p);
    }

    // Compute the final state.
    for p in &problem.schedule {
        map.remove(p);
    }
    for &p in &map {
        for dir in Point::DIRS {
            let q = p + dir;
            if map.contains(&q) {
                uf.merge(p, q);
            }
        }
    }

    // The map should be disconnected at the end.
    let start = Point::new(0, 0);
    let goal = Point::new(size - 1, size - 1);
    ensure!(
        uf.same(start, goal) != Some(true),
        "The map is connected at the end"
    );

    // Simulate the memory corruption in the reversed order.
    for &p in problem.schedule.iter().rev() {
        map.insert(p);
        for dir in Point::DIRS {
            let q = p + dir;
            if map.contains(&q) {
                uf.merge(p, q);
            }
        }
        if uf.same(start, goal) == Some(true) {
            return Ok(p);
        }
    }

    unreachable!()
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin().lock())?;
    let problem: Problem = input.parse()?;
    let answer = solve(&problem, 71)?;
    println!("{:?}", answer);
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
        let answer = solve(&problem, 7)?;
        assert_eq!(answer, Point::new(6, 1));
        Ok(())
    }
}
