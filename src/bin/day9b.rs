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

    for src_pos in (0..blocks.len()).rev() {
        loop {
            let move_file_len = blocks[src_pos].file_len;
            let mut moved = false;
            for dst_pos in 0..src_pos {
                if blocks[dst_pos].free_len >= move_file_len {
                    let mut move_block = blocks.remove(src_pos);
                    blocks[src_pos - 1].free_len += move_block.file_len + move_block.free_len;
                    move_block.free_len = blocks[dst_pos].free_len - move_block.file_len;
                    blocks[dst_pos].free_len = 0;
                    blocks.insert(dst_pos + 1, move_block);
                    moved = true;
                    break;
                }
            }
            if !moved {
                break;
            }
        }
    }

    let mut checksum = 0;
    let mut offset = 0;
    for block in &blocks {
        for _ in 0..block.file_len {
            checksum += offset * block.id;
            offset += 1;
        }
        offset += block.free_len;
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
        assert_eq!(answer, 2858);
        Ok(())
    }
}
