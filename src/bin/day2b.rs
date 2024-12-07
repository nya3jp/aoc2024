use std::str::FromStr;

use anyhow::{Error, Result};

#[derive(Clone, Debug)]
struct Problem {
    reports: Vec<Vec<i32>>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let reports = input
            .lines()
            .map(|line| -> Result<Vec<_>> {
                line.split_whitespace()
                    .map(|x| x.parse::<i32>().map_err(Error::new))
                    .collect()
            })
            .collect::<Result<_>>()?;
        Ok(Problem { reports })
    }
}

fn is_safe_forward(mut report: impl Iterator<Item = i32>) -> bool {
    let Some(mut last) = report.next() else {
        return false;
    };
    for next in report {
        if next - last > 3 || next - last < 1 {
            return false;
        }
        last = next;
    }
    true
}

fn is_safe(report: &[i32]) -> bool {
    is_safe_forward(report.iter().copied()) || is_safe_forward(report.iter().copied().rev())
}

fn is_roughly_safe(report: &[i32]) -> bool {
    if is_safe(report) {
        return true;
    }
    for i in 0..report.len() {
        let mut new_report: Vec<_> = report.to_vec();
        new_report.remove(i);
        if is_safe(&new_report) {
            return true;
        }
    }
    false
}

fn solve(problem: &Problem) -> Result<usize> {
    Ok(problem
        .reports
        .iter()
        .filter(|report| is_roughly_safe(report))
        .count())
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
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 4);
        Ok(())
    }
}
