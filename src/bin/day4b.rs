use std::str::FromStr;

use anyhow::{Error, Result};
use itertools::Itertools;

#[derive(Clone, Debug)]
struct Problem {
    map: Vec<Vec<char>>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let map = input
            .trim()
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect();
        Ok(Problem { map })
    }
}

fn solve(problem: &Problem) -> Result<usize> {
    let map = &problem.map;
    let n = map.len();
    let m = map[0].len();
    assert!(map.iter().all(|row| row.len() == m));

    let mut cnt = 0;
    for si in 1..n - 1 {
        for sj in 1..m - 1 {
            for di in -1..=1 {
                for dj in -1..=1 {
                    if di == 0 || dj == 0 {
                        continue;
                    }
                    if map[si][sj] == 'A'
                        && map[(si as isize + di) as usize][(sj as isize + dj) as usize] == 'M'
                        && map[(si as isize - dj) as usize][(sj as isize + di) as usize] == 'M'
                        && map[(si as isize - di) as usize][(sj as isize - dj) as usize] == 'S'
                        && map[(si as isize + dj) as usize][(sj as isize - di) as usize] == 'S'
                    {
                        cnt += 1;
                    }
                }
            }
        }
    }

    Ok(cnt)
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
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 9);
        Ok(())
    }
}
