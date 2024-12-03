use anyhow::Result;
use regex::Regex;

fn solve(program: &str) -> i32 {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    pattern
        .captures_iter(program)
        .map(|capture| {
            let a: i32 = capture.get(1).unwrap().as_str().parse().unwrap();
            let b: i32 = capture.get(2).unwrap().as_str().parse().unwrap();
            a * b
        })
        .sum()
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin().lock())?;
    let program = input.trim();
    let answer = solve(program);
    println!("{}", answer);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() -> Result<()> {
        let program = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        let answer = solve(program);
        assert_eq!(answer, 161);
        Ok(())
    }
}
