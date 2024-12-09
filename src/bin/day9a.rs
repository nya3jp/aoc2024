use std::str::FromStr;

use anyhow::{ensure, Context, Error, Result};

#[derive(Clone, Debug)]
struct Block {
    id: u64,
    file_len: u64,
    free_len: u64,
}

#[derive(Clone, Debug)]
struct Problem {
    blocks: Vec<Block>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let chars: Vec<char> = input.chars().collect();
        ensure!(chars.len() % 2 == 1, "Even number of characters");

        let mut blocks: Vec<Block> = Vec::new();
        for i in (0..chars.len()).step_by(2) {
            let id = (i / 2) as u64;
            let file_len: u64 = chars[i].to_digit(10).context("Not a digit")? as u64;
            ensure!(file_len > 0, "Zero file length");
            let free_len: u64 = chars
                .get(i + 1)
                .unwrap_or(&'0')
                .to_digit(10)
                .context("Not a digit")? as u64;
            blocks.push(Block {
                id,
                file_len,
                free_len,
            });
        }

        Ok(Problem { blocks })
    }
}

fn solve(problem: &Problem) -> Result<u64> {
    let mut blocks = problem.blocks.clone();
    let mut blocks = &mut blocks[..];
    let mut offset = 0;
    let mut checksum = 0;

    while !blocks.is_empty() {
        {
            let first_block = blocks.first_mut().unwrap();
            while first_block.file_len > 0 {
                checksum += offset * first_block.id;
                first_block.file_len -= 1;
                offset += 1;
            }
        }

        if blocks.len() == 1 {
            break;
        }

        let first_free_len = blocks.first().unwrap().free_len;
        let last_file_len = blocks.last().unwrap().file_len;
        let last_block_id = blocks.last().unwrap().id;
        let move_size = first_free_len.min(last_file_len);
        for _ in 0..move_size {
            checksum += offset * last_block_id;
            blocks.first_mut().unwrap().free_len -= 1;
            blocks.last_mut().unwrap().file_len -= 1;
            offset += 1;
        }

        if blocks.first().unwrap().free_len == 0 {
            blocks = &mut blocks[1..];
        }
        if blocks.last().unwrap().file_len == 0 {
            let n = blocks.len();
            blocks = &mut blocks[..n - 1];
        }
    }

    Ok(checksum)
}

fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin().lock())?;
    let problem: Problem = input.trim().parse()?;
    let answer = solve(&problem)?;
    println!("{}", answer);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() -> Result<()> {
        let input = "2333133121414131402";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, 1928);
        Ok(())
    }
}
