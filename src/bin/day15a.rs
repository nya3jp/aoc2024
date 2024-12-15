use std::{
    ops::{Add, Mul},
    str::FromStr,
};

use anyhow::{bail, ensure, Context, Error, Result};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cell {
    Empty,
    Block,
    Box,
}

impl TryFrom<char> for Cell {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Cell::Empty),
            '#' => Ok(Cell::Block),
            'O' => Ok(Cell::Box),
            _ => bail!("Invalid cell: {}", c),
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

        let mut robots = Vec::new();
        let map: Vec<Vec<Cell>> = map_str
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        '@' => {
                            robots.push(Point { i, j });
                            Ok(Cell::Empty)
                        }
                        _ => c.try_into(),
                    })
                    .collect::<Result<Vec<Cell>>>()
            })
            .collect::<Result<_>>()?;

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
        let mut len = 1;
        loop {
            match self.cell(self.robot + dir * len) {
                Cell::Block => return,
                Cell::Box => {
                    len += 1;
                }
                Cell::Empty => {
                    break;
                }
            }
        }

        *self.cell_mut(self.robot + dir * len) = Cell::Box;
        *self.cell_mut(self.robot + dir) = Cell::Empty;
        self.robot = self.robot + dir;
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
                        Cell::Box => eprint!("O"),
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
                if cell == Cell::Box {
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
    fn sample1() -> Result<()> {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 2028);
        Ok(())
    }

    #[test]
    fn sample2() -> Result<()> {
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
        assert_eq!(answer, 10092);
        Ok(())
    }
}
