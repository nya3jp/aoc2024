use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    ops::{Add, Mul},
    str::FromStr,
};

use anyhow::{ensure, Context, Error, Result};

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

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State {
    pub pos: Point,
    pub dir: Point,
}

fn search_paths(problem: &Problem) -> Result<(State, HashMap<State, HashSet<State>>)> {
    let mut queue: BinaryHeap<Reverse<(usize, State, State)>> = BinaryHeap::new();
    let mut finalized: HashSet<State> = HashSet::new();
    let mut known_best: HashMap<State, usize> = HashMap::new();
    let mut trace: HashMap<State, HashSet<State>> = HashMap::new();

    let start_state = State {
        pos: problem.start,
        dir: Point::RIGHT,
    };
    queue.push(Reverse((0, start_state, start_state)));
    known_best.insert(start_state, 0);

    while let Some(Reverse((current_cost, current_state, previous_state))) = queue.pop() {
        if current_cost == known_best[&current_state] {
            trace
                .entry(current_state)
                .or_default()
                .insert(previous_state);
        }
        if !finalized.insert(current_state) {
            continue;
        }

        for next_dir in Point::DIRS {
            let next_pos = current_state.pos + next_dir;
            if !problem.map.contains(&next_pos) {
                continue;
            }
            let next_cost = current_cost
                + 1
                + if next_dir != current_state.dir {
                    1000
                } else {
                    0
                };
            let next_state = State {
                pos: next_pos,
                dir: next_dir,
            };
            if let Some(&best) = known_best.get(&next_state) {
                if next_cost > best {
                    continue;
                }
            }
            queue.push(Reverse((next_cost, next_state, current_state)));
            known_best.insert(next_state, next_cost);
        }
    }

    let (_, goal_state) = Point::DIRS
        .iter()
        .copied()
        .filter_map(|dir| {
            let state = State {
                pos: problem.goal,
                dir,
            };
            let cost = known_best.get(&state)?;
            Some((cost, state))
        })
        .min()
        .context("No path found")?;

    Ok((goal_state, trace))
}

fn count_places(trace: &HashMap<State, HashSet<State>>, goal_state: State) -> usize {
    let mut stack: Vec<State> = vec![goal_state];
    let mut visits: HashSet<State> = HashSet::from([goal_state]);
    while let Some(current_state) = stack.pop() {
        if let Some(previous_states) = trace.get(&current_state) {
            for &previous_state in previous_states {
                if visits.insert(previous_state) {
                    stack.push(previous_state);
                }
            }
        }
    }
    let places: HashSet<Point> = visits.into_iter().map(|state| state.pos).collect();
    places.len()
}

fn solve(problem: &Problem) -> Result<usize> {
    let (goal_state, trace) = search_paths(problem)?;
    Ok(count_places(&trace, goal_state))
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
        assert_eq!(answer, 45);
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
        assert_eq!(answer, 64);
        Ok(())
    }
}
