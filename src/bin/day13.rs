#![feature(iter_array_chunks)]
use std::io::{stdin, BufRead};

use anyhow::anyhow;

#[derive(Debug, Clone, Copy)]
struct ClawMachine {
    pub a_button: (i64, i64),
    pub b_button: (i64, i64),
    pub target_position: (i64, i64),
}
impl ClawMachine {
    fn from_lines(lines: &[String; 3]) -> anyhow::Result<Self> {
        let a_button = ClawMachine::parse_coordinate(&lines[0])?;
        let b_button = ClawMachine::parse_coordinate(&lines[1])?;
        let target_position = ClawMachine::parse_coordinate(&lines[2])?;
        let claw_machine = ClawMachine {
            a_button,
            b_button,
            target_position,
        };
        Ok(claw_machine)
    }
    fn parse_coordinate(line: &str) -> anyhow::Result<(i64, i64)> {
        if let Some((_discard, rest)) = line.split_once('X') {
            if let Some((x, rest)) = rest.split_once(',') {
                let x = x.strip_prefix('=').unwrap_or(x).parse::<i64>()?;
                if let Some((_discard, y)) = rest.split_once('Y') {
                    let y = y.strip_prefix('=').unwrap_or(y).parse::<i64>()?;
                    Ok((x, y))
                } else {
                    Err(anyhow!("The X coordinate must finish with ',': {}", line))
                }
            } else {
                Err(anyhow!("The X coordinate must finish with ',': {}", line))
            }
        } else {
            Err(anyhow!("Can not split into coords: {}", line))
        }
    }

    fn min_tokens(&self) -> Option<(i64, i64)> {
        if let Some(b_times) = self
            .a_button
            .1
            .checked_mul(self.target_position.0)
            .and_then(|b_part: i64| {
                self.a_button
                    .0
                    .checked_mul(self.target_position.1)
                    .and_then(|b_part_1| b_part.checked_sub(b_part_1))
            })
            .and_then(|b_part| {
                self.b_button
                    .0
                    .checked_mul(self.a_button.1)
                    .and_then(|b_part_1| {
                        self.b_button
                            .1
                            .checked_mul(self.a_button.0)
                            .and_then(|b_part_2| b_part_1.checked_sub(b_part_2))
                    })
                    .and_then(|b_part_1| b_part.checked_div(b_part_1))
            })
        {
            if let Some(a_times) = self
                .b_button
                .0
                .checked_mul(b_times)
                .and_then(|a_part| self.target_position.0.checked_sub(a_part))
                .and_then(|a_part| a_part.checked_div(self.a_button.0))
            {
                if a_times * self.a_button.0 + b_times * self.b_button.0 == self.target_position.0
                    && a_times * self.a_button.1 + b_times * self.b_button.1
                        == self.target_position.1
                {
                    // println!("{a_times:#?} {b_times:#?}");
                    return Some((a_times, b_times));
                }
            }
        }
        None
    }

    fn change_target(&self, change: i64) -> Self {
        Self {
            a_button: self.a_button,
            b_button: self.b_button,
            target_position: (
                self.target_position.0 + change,
                self.target_position.1 + change,
            ),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let claws = stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .array_chunks()
        .map(|lines| ClawMachine::from_lines(&lines))
        .collect::<anyhow::Result<Vec<ClawMachine>>>()?;
    let min_tokens: i64 = claws
        .iter()
        .filter_map(|claw| claw.min_tokens())
        .map(|(a_times, b_times)| a_times * 3 + b_times)
        // .inspect(|tokens| println!("tokens: {tokens}"))
        .sum();
    println!("{:?}", min_tokens);
    let min_tokens: i64 = claws
        .iter()
        .map(|claw| claw.change_target(10000000000000))
        .filter_map(|claw| claw.min_tokens())
        .map(|(a_times, b_times)| a_times * 3 + b_times)
        // .inspect(|tokens| println!("tokens: {tokens}"))
        .sum();
    println!("{:?}", min_tokens);
    Ok(())
}
