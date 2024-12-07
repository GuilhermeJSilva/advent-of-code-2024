use anyhow::{anyhow, Context, Error, Result};
use std::{
    io::{stdin, BufRead},
    str::FromStr,
};

#[derive(Debug)]
struct Equation {
    target: usize,
    operands: Vec<usize>,
}

impl FromStr for Equation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((target, operands)) = s.split_once(": ") {
            let target = target.parse()?;
            let operands: Result<Vec<usize>, _> = operands
                .split_whitespace()
                .map(|operand| operand.parse::<usize>())
                .collect();
            let operands = operands?;

            return Ok(Equation { target, operands });
        }
        return Err(anyhow!("failed to split on :"));
    }
}

impl Equation {
    pub fn count_solutions(&self, include_concatenation: bool) -> usize {
        self.count_solutions_internal(1, self.operands[0], include_concatenation)
    }

    fn count_solutions_internal(
        &self,
        next_operand: usize,
        value: usize,
        include_concatenation: bool,
    ) -> usize {
        if next_operand >= self.operands.len() {
            if value == self.target {
                1
            } else {
                0
            }
        } else if value > self.target {
            0
        } else {
            self.count_solutions_internal(
                next_operand + 1,
                value + self.operands[next_operand],
                include_concatenation,
            ) + self.count_solutions_internal(
                next_operand + 1,
                value * self.operands[next_operand],
                include_concatenation,
            ) + if include_concatenation {
                self.count_solutions_internal(
                    next_operand + 1,
                    value * 10usize.pow(self.operands[next_operand].ilog10() + 1)
                        + self.operands[next_operand],
                    include_concatenation,
                )
            } else {
                0
            }
        }
    }
}

fn main() -> Result<()> {
    let equations: Vec<Equation> = stdin()
        .lock()
        .lines()
        .map(|line| {
            line.with_context(|| format!("failed to get line from stdin"))
                .and_then(|line| Equation::from_str(&line))
        })
        .collect::<Result<Vec<Equation>>>()?;
    let calibration_result: usize = equations
        .iter()
        .filter(|equation| equation.count_solutions(false) != 0)
        .map(|equation| equation.target)
        .sum();
    println!("{:?}", calibration_result);
    let calibration_result: usize = equations
        .iter()
        .filter(|equation| equation.count_solutions(true) != 0)
        .map(|equation| equation.target)
        .sum();
    println!("{:?}", calibration_result);
    Ok(())
}
