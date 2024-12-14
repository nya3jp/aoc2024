use std::str::FromStr;

use anyhow::{Context, Error, Result};
use regex::Regex;

#[derive(Clone, Debug)]
struct Robot {
    pub x: i32,
    pub y: i32,
    pub vx: i32,
    pub vy: i32,
}

impl FromStr for Robot {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let pattern = Regex::new(r"^p=(-?[0-9]+),(-?[0-9]+) v=(-?[0-9]+),(-?[0-9]+)$")?;
        let c = pattern.captures(input).context("Regex mismatch")?;
        let x: i32 = c.get(1).unwrap().as_str().parse()?;
        let y: i32 = c.get(2).unwrap().as_str().parse()?;
        let vx: i32 = c.get(3).unwrap().as_str().parse()?;
        let vy: i32 = c.get(4).unwrap().as_str().parse()?;
        Ok(Robot { x, y, vx, vy })
    }
}

impl Robot {
    pub fn tick(&mut self, w: i32, h: i32) {
        self.x = (self.x + self.vx).rem_euclid(w);
        self.y = (self.y + self.vy).rem_euclid(h);
    }
}

#[derive(Clone, Debug)]
struct Problem {
    pub robots: Vec<Robot>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let robots: Vec<Robot> = input
            .trim()
            .lines()
            .map(|s| s.parse::<Robot>())
            .collect::<Result<_>>()?;
        Ok(Problem { robots })
    }
}

fn compute_safety_factor(robots: &[Robot], w: i32, h: i32) -> usize {
    let mut counts = [[0; 2]; 2];
    for robot in robots {
        if robot.x == w / 2 || robot.y == h / 2 {
            continue;
        }
        counts[(robot.x / ((w + 1) / 2)) as usize][(robot.y / ((h + 1) / 2)) as usize] += 1;
    }
    counts[0][0] * counts[0][1] * counts[1][0] * counts[1][1]
}

fn solve(problem: &Problem, w: i32, h: i32, ticks: usize) -> Result<usize> {
    let mut robots = problem.robots.clone();
    for _ in 0..ticks {
        for robot in &mut robots {
            robot.tick(w, h);
        }
    }
    Ok(compute_safety_factor(&robots, w, h))
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin().lock())?;
    let problem: Problem = input.trim().parse()?;
    let answer = solve(&problem, 101, 103, 100)?;
    println!("{}", answer);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() -> Result<()> {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem, 11, 7, 100)?;
        assert_eq!(answer, 12);
        Ok(())
    }
}
