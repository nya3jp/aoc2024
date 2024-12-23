use std::{
    fmt::Display,
    ops::{Add, Sub},
    str::FromStr,
};

use anyhow::{bail, Context, Error, Result};
use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Point {
    pub i: isize,
    pub j: isize,
}

impl Point {
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

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            i: self.i - other.i,
            j: self.j - other.j,
        }
    }
}

trait Button: Copy {
    fn initial_position() -> Point;
    fn as_char(self) -> char;
    fn position(self) -> Point;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NumericButton {
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    Activate,
}

impl Button for NumericButton {
    fn initial_position() -> Point {
        Self::Activate.position()
    }

    fn as_char(self) -> char {
        match self {
            Self::N0 => '0',
            Self::N1 => '1',
            Self::N2 => '2',
            Self::N3 => '3',
            Self::N4 => '4',
            Self::N5 => '5',
            Self::N6 => '6',
            Self::N7 => '7',
            Self::N8 => '8',
            Self::N9 => '9',
            Self::Activate => 'A',
        }
    }

    fn position(self) -> Point {
        match self {
            Self::N7 => Point::new(-3, 0),
            Self::N8 => Point::new(-3, 1),
            Self::N9 => Point::new(-3, 2),
            Self::N4 => Point::new(-2, 0),
            Self::N5 => Point::new(-2, 1),
            Self::N6 => Point::new(-2, 2),
            Self::N1 => Point::new(-1, 0),
            Self::N2 => Point::new(-1, 1),
            Self::N3 => Point::new(-1, 2),
            Self::N0 => Point::new(0, 1),
            Self::Activate => Point::new(0, 2),
        }
    }
}

impl TryFrom<char> for NumericButton {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            '0' => Ok(Self::N0),
            '1' => Ok(Self::N1),
            '2' => Ok(Self::N2),
            '3' => Ok(Self::N3),
            '4' => Ok(Self::N4),
            '5' => Ok(Self::N5),
            '6' => Ok(Self::N6),
            '7' => Ok(Self::N7),
            '8' => Ok(Self::N8),
            '9' => Ok(Self::N9),
            'A' => Ok(Self::Activate),
            _ => bail!("Invalid character for numeric button: {}", c),
        }
    }
}

impl Display for NumericButton {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DirectionalButton {
    Up,
    Left,
    Down,
    Right,
    Activate,
}

impl DirectionalButton {
    pub fn as_delta(self) -> Option<Point> {
        match self {
            Self::Up => Some(Point::new(-1, 0)),
            Self::Left => Some(Point::new(0, -1)),
            Self::Down => Some(Point::new(1, 0)),
            Self::Right => Some(Point::new(0, 1)),
            Self::Activate => None,
        }
    }
}

impl Button for DirectionalButton {
    fn initial_position() -> Point {
        Self::Activate.position()
    }

    fn as_char(self) -> char {
        match self {
            Self::Up => '^',
            Self::Left => '<',
            Self::Down => 'v',
            Self::Right => '>',
            Self::Activate => 'A',
        }
    }

    fn position(self) -> Point {
        match self {
            Self::Up => Point::new(0, 1),
            Self::Activate => Point::new(0, 2),
            Self::Left => Point::new(1, 0),
            Self::Down => Point::new(1, 1),
            Self::Right => Point::new(1, 2),
        }
    }
}

impl TryFrom<char> for DirectionalButton {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            '^' => Ok(Self::Up),
            '<' => Ok(Self::Left),
            'v' => Ok(Self::Down),
            '>' => Ok(Self::Right),
            'A' => Ok(Self::Activate),
            _ => bail!("Invalid character for directional button: {}", c),
        }
    }
}

impl Display for DirectionalButton {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

#[derive(Clone, Debug)]
struct Problem {
    pub codes: Vec<(Vec<NumericButton>, u32)>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let codes = input
            .lines()
            .map(|s| {
                let s = s.strip_suffix('A').context("Code must end with A")?;
                let factor: u32 = s.parse()?;
                let mut code = s
                    .chars()
                    .map(|c| c.try_into())
                    .collect::<Result<Vec<_>>>()?;
                code.push(NumericButton::Activate);
                Ok((code, factor))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Problem { codes })
    }
}

fn cost_button(current_position: Point, next_position: Point, n: usize) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut button_set = Vec::new();
    let diff = next_position - current_position;
    if diff.i > 0 {
        button_set.extend(vec![DirectionalButton::Down; diff.i as usize]);
    }
    if diff.i < 0 {
        button_set.extend(vec![DirectionalButton::Up; -diff.i as usize]);
    }
    if diff.j > 0 {
        button_set.extend(vec![DirectionalButton::Right; diff.j as usize]);
    }
    if diff.j < 0 {
        button_set.extend(vec![DirectionalButton::Left; -diff.j as usize]);
    }
    let k = button_set.len();
    button_set
        .into_iter()
        .permutations(k)
        .filter_map(|mut buttons| {
            // Filter moves that pass through (0, 0).
            let mut simulated_position = current_position;
            for button in &buttons {
                simulated_position = simulated_position + button.as_delta().unwrap();
                if simulated_position == Point::new(0, 0) {
                    return None;
                }
            }
            buttons.push(DirectionalButton::Activate);
            Some(cost_buttons(&buttons, n - 1))
        })
        .min()
        .unwrap()
}

fn cost_buttons<T: Button>(buttons: &[T], n: usize) -> u32 {
    let mut cost = 0;
    let mut current_position = T::initial_position();
    for button in buttons {
        let next_position = button.position();
        cost += cost_button(current_position, next_position, n);
        current_position = next_position;
    }
    cost
}

fn solve(problem: &Problem) -> Result<u32> {
    Ok(problem
        .codes
        .iter()
        .map(|(code, factor)| cost_buttons(code, 3) * factor)
        .sum())
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
        let input = "029A
980A
179A
456A
379A
";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 126384);
        Ok(())
    }
}
