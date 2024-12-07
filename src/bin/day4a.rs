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
    for si in 0..n {
        for sj in 0..m {
            for di in -1..=1 {
                for dj in -1..=1 {
                    if di == 0 && dj == 0 {
                        continue;
                    }

                    let ei = si as isize + di * 3;
                    let ej = sj as isize + dj * 3;
                    if ei < 0 || ei >= n as isize || ej < 0 || ej >= m as isize {
                        continue;
                    }

                    let word = (0..4)
                        .map(|k| {
                            let i = (si as isize + di * k) as usize;
                            let j = (sj as isize + dj * k) as usize;
                            map[i][j]
                        })
                        .collect::<String>();
                    if word == "XMAS" {
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
        assert_eq!(answer, 18);
        Ok(())
    }
}
