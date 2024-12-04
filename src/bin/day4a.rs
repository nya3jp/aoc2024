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
        assert_eq!(answer, 18);
        Ok(())
    }
}
