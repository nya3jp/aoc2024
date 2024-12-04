use anyhow::Result;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn solve(map: &[Vec<char>]) -> usize {
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

    cnt
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin().lock())?;
    let map = parse_input(input.trim());
    let answer = solve(&map);
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
        let map = parse_input(input);
        let answer = solve(&map);
        assert_eq!(answer, 9);
        Ok(())
    }
}
