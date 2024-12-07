use anyhow::{Context, Result};

#[derive(Clone, Debug)]
struct Equation {
    target: u64,
    factors: Vec<u64>,
}

#[derive(Clone, Debug)]
struct Problem {
    equations: Vec<Equation>,
}

fn parse_input(input: &str) -> Result<Problem> {
    let equations = input
        .trim()
        .lines()
        .map(|line| {
            let (target_str, factors_str) = line.split_once(": ").context("Invalid input")?;
            let target = target_str.parse()?;
            let factors = factors_str
                .split_ascii_whitespace()
                .map(|x| x.parse::<u64>())
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Equation { target, factors })
        })
        .collect::<Result<_>>()?;
    Ok(Problem { equations })
}

fn search(current: u64, target: u64, factors: &[u64]) -> bool {
    if factors.is_empty() {
        return current == target;
    }
    if current > target {
        return false;
    }
    search(current + factors[0], target, &factors[1..])
        || search(current * factors[0], target, &factors[1..])
}

fn can_produce(equation: &Equation) -> bool {
    search(equation.factors[0], equation.target, &equation.factors[1..])
}

fn solve(problem: &Problem) -> u64 {
    problem
        .equations
        .iter()
        .filter(|equation| can_produce(equation))
        .map(|equation| equation.target)
        .sum()
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin().lock())?;
    let problem = parse_input(input.trim())?;
    let answer = solve(&problem);
    println!("{}", answer);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() -> Result<()> {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;
        let problem = parse_input(input)?;
        let answer = solve(&problem);
        assert_eq!(answer, 3749);
        Ok(())
    }
}
