use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    str::FromStr,
    vec,
};

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

fn niceness(robots: &[Robot], w: i32, h: i32) -> usize {
    let mut canvas = vec![vec!['.'; w as usize]; h as usize];
    for robot in robots {
        canvas[robot.y as usize][robot.x as usize] = '#';
    }

    let mut niceness = 0;
    for x in 1..(w - 1) {
        for y in 1..(h - 1) {
            if canvas[y as usize][x as usize] == '#' {
                let mut nice = false;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if canvas[(y + dy) as usize][(x + dx) as usize] == '#' {
                            nice = true;
                        }
                    }
                }
                if nice {
                    niceness += 1;
                }
            }
        }
    }

    niceness
}

#[derive(Clone, Debug)]
struct Entry {
    pub niceness: usize,
    pub tick: usize,
    pub robots: Vec<Robot>,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.niceness == other.niceness
    }
}

impl Eq for Entry {}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.niceness.cmp(&other.niceness)
    }
}

fn draw(robots: &[Robot], w: i32, h: i32, tick: usize) {
    let mut canvas = vec![vec!['.'; w as usize]; h as usize];
    for robot in robots {
        canvas[robot.y as usize][robot.x as usize] = '#';
    }
    println!(
        "======================================================== {}",
        tick
    );
    for row in canvas {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
}

fn tick_all(robots: &mut [Robot], w: i32, h: i32) {
    for robot in robots {
        robot.tick(w, h);
    }
}

fn solve(problem: &Problem, w: i32, h: i32) -> Result<()> {
    let mut robots = problem.robots.clone();
    let mut ranked = BinaryHeap::new();
    for tick in 0..10000 {
        ranked.push(Reverse(Entry {
            niceness: niceness(&robots, w, h),
            tick,
            robots: robots.clone(),
        }));
        if ranked.len() > 10 {
            ranked.pop();
        }
        tick_all(&mut robots, w, h);
    }
    let bests = ranked.into_sorted_vec();
    for best in bests {
        draw(&best.0.robots, w, h, best.0.tick);
    }
    Ok(())
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin().lock())?;
    let problem: Problem = input.trim().parse()?;
    solve(&problem, 101, 103)?;
    Ok(())
}
