use std::str::FromStr;

use anyhow::{bail, Context, Error, Result};
use regex::Regex;

#[derive(Clone, Debug)]
struct Regs {
    pub a: u64,
    pub b: u64,
    pub c: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Combo {
    Literal(u64),
    RegA,
    RegB,
    RegC,
    Reserved,
}

impl TryFrom<u8> for Combo {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0..=3 => Ok(Combo::Literal(value as u64)),
            4 => Ok(Combo::RegA),
            5 => Ok(Combo::RegB),
            6 => Ok(Combo::RegC),
            7 => Ok(Combo::Reserved),
            _ => bail!("Invalid combo value: {}", value),
        }
    }
}

impl Combo {
    pub fn eval(self, regs: &Regs) -> u64 {
        match self {
            Combo::Literal(value) => value,
            Combo::RegA => regs.a,
            Combo::RegB => regs.b,
            Combo::RegC => regs.c,
            Combo::Reserved => panic!("Combo::Reserved"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Inst {
    Adv(Combo),
    Bxl(u8),
    Bst(Combo),
    Jnz(u8),
    Bxc,
    Out(Combo),
    Bdv(Combo),
    Cdv(Combo),
}

impl TryFrom<(u8, u8)> for Inst {
    type Error = Error;

    fn try_from((opcode, operand): (u8, u8)) -> Result<Self> {
        match opcode {
            0 => Ok(Inst::Adv(operand.try_into()?)),
            1 => Ok(Inst::Bxl(operand)),
            2 => Ok(Inst::Bst(operand.try_into()?)),
            3 => Ok(Inst::Jnz(operand)),
            4 => Ok(Inst::Bxc),
            5 => Ok(Inst::Out(operand.try_into()?)),
            6 => Ok(Inst::Bdv(operand.try_into()?)),
            7 => Ok(Inst::Cdv(operand.try_into()?)),
            _ => bail!("Invalid opcode: {}", opcode),
        }
    }
}

#[derive(Clone, Debug)]
struct Machine {
    pub regs: Regs,
    pub ip: usize,
    pub program: Vec<u8>,
    pub output: Vec<u64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StepResult {
    Continue,
    Halt,
}

impl Machine {
    pub fn new(regs: Regs, program: Vec<u8>) -> Self {
        Machine {
            regs,
            ip: 0,
            program,
            output: Vec::new(),
        }
    }

    pub fn execute_step(&mut self) -> Result<StepResult> {
        if self.ip + 1 >= self.program.len() {
            return Ok(StepResult::Halt);
        }
        let inst = Inst::try_from((self.program[self.ip], self.program[self.ip + 1]))?;
        match inst {
            Inst::Adv(operand) => {
                self.regs.a >>= operand.eval(&self.regs);
                self.ip += 2;
            }
            Inst::Bxl(operand) => {
                self.regs.b ^= operand as u64;
                self.ip += 2;
            }
            Inst::Bst(operand) => {
                self.regs.b = operand.eval(&self.regs) & 7;
                self.ip += 2;
            }
            Inst::Jnz(operand) => {
                if self.regs.a == 0 {
                    self.ip += 2;
                } else {
                    self.ip = operand as usize;
                }
            }
            Inst::Bxc => {
                self.regs.b ^= self.regs.c;
                self.ip += 2;
            }
            Inst::Out(operand) => {
                self.output.push(operand.eval(&self.regs) % 8);
                self.ip += 2;
            }
            Inst::Bdv(operand) => {
                self.regs.b = self.regs.a >> operand.eval(&self.regs);
                self.ip += 2;
            }
            Inst::Cdv(operand) => {
                self.regs.c = self.regs.a >> operand.eval(&self.regs);
                self.ip += 2;
            }
        }
        Ok(StepResult::Continue)
    }

    pub fn execute_to_halt(&mut self) -> Result<()> {
        while self.execute_step()? == StepResult::Continue {}
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Problem {
    pub regs: Regs,
    pub program: Vec<u8>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(problem_str: &str) -> Result<Self> {
        let pattern = Regex::new(
            r"^Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: ([,0-9]+)\n?$",
        )
        .unwrap();
        let captures = pattern.captures(problem_str).context("Invalid input")?;

        let regs = Regs {
            a: captures[1].parse()?,
            b: captures[2].parse()?,
            c: captures[3].parse()?,
        };
        let program: Vec<u8> = captures[4]
            .split(',')
            .map(|s| s.parse::<u8>())
            .collect::<Result<_, _>>()?;

        Ok(Problem { regs, program })
    }
}

fn solve(problem: &Problem) -> Result<String> {
    let mut machine = Machine::new(problem.regs.clone(), problem.program.clone());
    machine.execute_to_halt()?;
    Ok(machine
        .output
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(","))
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
    fn microsample1() -> Result<()> {
        let mut machine = Machine::new(Regs { a: 0, b: 0, c: 9 }, vec![2, 6]);
        machine.execute_to_halt()?;
        assert_eq!(machine.regs.b, 1);
        Ok(())
    }

    #[test]
    fn microsample2() -> Result<()> {
        let mut machine = Machine::new(Regs { a: 10, b: 0, c: 0 }, vec![5, 0, 5, 1, 5, 4]);
        machine.execute_to_halt()?;
        assert_eq!(machine.output, vec![0, 1, 2]);
        Ok(())
    }

    #[test]
    fn microsample3() -> Result<()> {
        let mut machine = Machine::new(
            Regs {
                a: 2024,
                b: 0,
                c: 0,
            },
            vec![0, 1, 5, 4, 3, 0],
        );
        machine.execute_to_halt()?;
        assert_eq!(machine.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(machine.regs.a, 0);
        Ok(())
    }

    #[test]
    fn microsample4() -> Result<()> {
        let mut machine = Machine::new(Regs { a: 0, b: 29, c: 0 }, vec![1, 7]);
        machine.execute_to_halt()?;
        assert_eq!(machine.regs.b, 26);
        Ok(())
    }

    #[test]
    fn microsample5() -> Result<()> {
        let mut machine = Machine::new(
            Regs {
                a: 0,
                b: 2024,
                c: 43690,
            },
            vec![4, 0],
        );
        machine.execute_to_halt()?;
        assert_eq!(machine.regs.b, 44354);
        Ok(())
    }

    #[test]
    fn sample() -> Result<()> {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, "4,6,3,5,6,3,5,2,1,0");
        Ok(())
    }

    #[test]
    fn sample_quine() -> Result<()> {
        let input = "Register A: 156985331222018
Register B: 0
Register C: 0

Program: 2,4,1,4,7,5,4,1,1,4,5,5,0,3,3,0
";
        let problem: Problem = input.parse()?;
        let answer = solve(&problem)?;
        assert_eq!(answer, "2,4,1,4,7,5,4,1,1,4,5,5,0,3,3,0");
        Ok(())
    }
}
