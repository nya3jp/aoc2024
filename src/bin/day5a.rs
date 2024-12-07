use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Context, Result};
use itertools::Itertools;

struct Problem {
    rules: Vec<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

fn parse_input(input: &str) -> Result<Problem> {
    let (rules_str, updates_str) = input.split_once("\n\n").context("Parse error")?;
    let rules = rules_str
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('|').context("Parse error")?;
            Ok((a.parse()?, b.parse()?))
        })
        .collect::<Result<_>>()?;
    let updates = updates_str
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<_, _>>()?;
    Ok(Problem { rules, updates })
}

fn is_right_order(update: &[u32], rules_index: &BTreeMap<u32, Vec<u32>>) -> bool {
    let mut seen: BTreeSet<u32> = BTreeSet::new();
    for &a in update {
        if let Some(bs) = rules_index.get(&a) {
            if bs.iter().any(|b| seen.contains(b)) {
                return false;
            }
        }
        seen.insert(a);
    }
    true
}

fn solve(problem: &Problem) -> Result<u32> {
    let rules_index: BTreeMap<u32, Vec<u32>> = problem
        .rules
        .iter()
        .sorted()
        .chunk_by(|(a, _)| a)
        .into_iter()
        .map(|(a, v)| (*a, v.map(|(_, b)| *b).collect_vec()))
        .collect();

    Ok(problem
        .updates
        .iter()
        .filter(|update| is_right_order(update, &rules_index))
        .map(|update| update[update.len() / 2])
        .sum())
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin().lock())?;
    let map = parse_input(input.trim())?;
    let answer = solve(&map)?;
    println!("{}", answer);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() -> Result<()> {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;
        let problem = parse_input(input)?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 143);
        Ok(())
    }
}
