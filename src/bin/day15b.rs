use std::{
    collections::HashSet,
    ops::{Add, Mul},
    str::FromStr,
};

use anyhow::{bail, ensure, Context, Error, Result};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cell {
    Empty,
    Block,
    BoxLeft,
    BoxRight,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WideCell {
    Empty,
    Block,
    Box,
}

impl TryFrom<char> for WideCell {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            '.' => Ok(WideCell::Empty),
            '#' => Ok(WideCell::Block),
            'O' => Ok(WideCell::Box),
            _ => bail!("Invalid cell: {}", c),
        }
    }
}

impl WideCell {
    pub fn expand(self) -> [Cell; 2] {
        match self {
            WideCell::Empty => [Cell::Empty, Cell::Empty],
            WideCell::Block => [Cell::Block, Cell::Block],
            WideCell::Box => [Cell::BoxLeft, Cell::BoxRight],
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
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
    pub map: Vec<Vec<Cell>>,
    pub robot: Point,
    pub moves: Vec<Point>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let (map_str, moves_str) = input.split_once("\n\n").context("Invalid input")?;

        let mut raw_robots = Vec::new();
        let raw_map: Vec<Vec<WideCell>> = map_str
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        '@' => {
                            raw_robots.push(Point { i, j });
                            Ok(WideCell::Empty)
                        }
                        _ => c.try_into(),
                    })
                    .collect::<Result<Vec<WideCell>>>()
            })
            .collect::<Result<_>>()?;

        let robots = raw_robots
            .into_iter()
            .map(|p| Point { i: p.i, j: p.j * 2 })
            .collect::<Vec<Point>>();
        let map: Vec<Vec<Cell>> = raw_map
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .flat_map(WideCell::expand)
                    .collect::<Vec<Cell>>()
            })
            .collect();

        ensure!(robots.len() == 1, "Invalid number of robots");
        let robot = robots[0];

        let moves: Vec<Point> = moves_str
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .map(|c| match c {
                '^' => Ok(Point::UP),
                '>' => Ok(Point::RIGHT),
                'v' => Ok(Point::DOWN),
                '<' => Ok(Point::LEFT),
                _ => bail!("Invalid move: {}", c),
            })
            .collect::<Result<_>>()?;

        Ok(Problem { map, robot, moves })
    }
}

#[derive(Clone, Debug)]
struct State {
    pub map: Vec<Vec<Cell>>,
    pub robot: Point,
}

impl State {
    pub fn cell(&self, p: Point) -> Cell {
        self.map[p.i][p.j]
    }

    pub fn cell_mut(&mut self, p: Point) -> &mut Cell {
        &mut self.map[p.i][p.j]
    }

    pub fn make_move(&mut self, dir: Point) {
        let mut pushes = HashSet::new();
        if !self.attempt_push(self.robot + dir, dir, &mut pushes) {
            return;
        };

        let mut new_state = self.clone();
        for &p in &pushes {
            *new_state.cell_mut(p) = Cell::Empty;
        }
        for &p in &pushes {
            *new_state.cell_mut(p + dir) = self.cell(p);
        }
        new_state.robot = self.robot + dir;
        *self = new_state;
    }

    fn attempt_push(&self, p: Point, dir: Point, pushes: &mut HashSet<Point>) -> bool {
        if pushes.contains(&p) {
            return true;
        }

        let q = match self.cell(p) {
            Cell::Empty => return true,
            Cell::Block => return false,
            Cell::BoxLeft => p + Point::RIGHT,
            Cell::BoxRight => p + Point::LEFT,
        };

        pushes.insert(p);
        pushes.insert(q);

        self.attempt_push(p + dir, dir, pushes) && self.attempt_push(q + dir, dir, pushes)
    }

    #[allow(unused)]
    pub fn dump(&self) {
        for (i, row) in self.map.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if self.robot == (Point { i, j }) {
                    eprint!("@");
                } else {
                    match cell {
                        Cell::Empty => eprint!("."),
                        Cell::Block => eprint!("#"),
                        Cell::BoxLeft => eprint!("["),
                        Cell::BoxRight => eprint!("]"),
                    }
                }
            }
            eprintln!();
        }
        eprintln!();
    }
}

fn compute_gps_sum(map: &[Vec<Cell>]) -> usize {
    map.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &cell)| {
                if cell == Cell::BoxLeft {
                    Some(i * 100 + j)
                } else {
                    None
                }
            })
        })
        .sum()
}

fn solve(problem: &Problem) -> Result<usize> {
    let mut state = State {
        map: problem.map.clone(),
        robot: problem.robot,
    };

    for &dir in &problem.moves {
        state.make_move(dir);
        // state.dump();
    }

    state.dump();

    Ok(compute_gps_sum(&state.map))
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
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 9021);
        Ok(())
    }
}
