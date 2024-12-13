use std::{
    collections::{HashMap, HashSet},
    ops::Add,
    str::FromStr,
};

use anyhow::{Error, Result};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Position {
    pub i: i32,
    pub j: i32,
}

impl Position {
    const ADJACENTS: [Position; 4] = [
        Position { i: -1, j: 0 },
        Position { i: 0, j: 1 },
        Position { i: 1, j: 0 },
        Position { i: 0, j: -1 },
    ];
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            i: self.i + other.i,
            j: self.j + other.j,
        }
    }
}

#[derive(Clone, Debug)]
struct Problem {
    map: HashMap<Position, char>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut map = HashMap::new();
        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                map.insert(
                    Position {
                        i: i as i32,
                        j: j as i32,
                    },
                    c,
                );
            }
        }
        Ok(Problem { map })
    }
}

fn compute_region_price(
    map: &HashMap<Position, char>,
    start: Position,
    c: char,
    seen: &mut HashSet<Position>,
) -> Result<u32> {
    let mut area = 0;
    let mut touches = 0;
    let mut stack = vec![start];
    seen.insert(start);

    while let Some(current) = stack.pop() {
        area += 1;
        for adj in Position::ADJACENTS {
            let next = current + adj;
            if map.get(&next) != Some(&c) {
                continue;
            }
            touches += 1;
            if seen.insert(next) {
                stack.push(next);
            }
        }
    }

    let perimeter = area * 4 - touches;
    Ok(area * perimeter)
}

fn solve(problem: &Problem) -> Result<u32> {
    let mut price = 0;
    let mut seen = HashSet::new();
    for (&pos, &c) in &problem.map {
        if !seen.contains(&pos) {
            price += compute_region_price(&problem.map, pos, c, &mut seen)?;
        }
    }
    Ok(price)
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin().lock())?;
    let problem: Problem = input.trim().parse()?;
    let answer = solve(&problem)?;
    println!("{}", answer);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "AAAA
BBCD
BBCC
EEEC
";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 140);
        Ok(())
    }

    #[test]
    fn sample2() -> Result<()> {
        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 772);
        Ok(())
    }

    #[test]
    fn sample3() -> Result<()> {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 1930);
        Ok(())
    }
}
