use std::{
    collections::{BTreeSet, HashMap},
    io::{stdin, BufRead},
    num::ParseIntError,
};

use anyhow::{Error, Result};

struct Memoizer {
    memory: HashMap<(usize, usize), usize>,
}

impl Memoizer {
    fn count_after_steps(&mut self, value: usize, max_step: usize) -> usize {
        if max_step == 0 {
            return 1;
        }

        let memory_key = (value, max_step);
        if let Some(count) = self.memory.get(&memory_key) {
            return *count;
        }

        if value == 0 {
            let count = self.count_after_steps(1, max_step - 1);
            self.memory.insert(memory_key, count);
            return count;
        }

        let n_digits = value.ilog10() + 1;
        if n_digits % 2 == 0 {
            let mask = 10usize.pow(n_digits / 2);
            let count = self.count_after_steps(value / mask, max_step - 1)
                + self.count_after_steps(value % mask, max_step - 1);
            self.memory.insert(memory_key, count);
            count
        } else {
            let count = self.count_after_steps(value * 2024, max_step - 1);
            self.memory.insert(memory_key, count);
            count
        }
    }
}

fn explore_from_zero() -> BTreeSet<usize> {
    let mut current_values: Vec<usize> = Vec::new();

    let mut explored_values: BTreeSet<usize> = BTreeSet::new();
    explored_values.insert(0);
    current_values.push(1);
    while let Some(value) = current_values.pop() {
        if explored_values.contains(&value) {
            continue;
        }
        explored_values.insert(value);
        let n_digits = value.ilog10() + 1;
        if n_digits % 2 == 0 {
            let mask = 10usize.pow(n_digits / 2);
            current_values.push(value / mask);
            current_values.push(value % mask);
        } else {
            current_values.push(value * 2024);
        }
    }

    explored_values
}

#[allow(dead_code)]
fn first_try_count_after_steps(start_value: usize, steps: usize) -> usize {
    let mut current_values: Vec<(usize, usize)> = Vec::new();
    current_values.push((start_value, 0));
    let mut count = 0;
    while let Some((value, step)) = current_values.pop() {
        // while !current_values.is_empty() {
        //     let (value, step) = current_values.remove(0);
        if steps == step {
            count += 1;
            continue;
        }

        if value == 0 {
            current_values.push((1, step + 1));
            continue;
        }

        let n_digits = value.ilog10() + 1;
        if n_digits % 2 == 0 {
            let mask = 10usize.pow(n_digits / 2);
            current_values.push((value / mask, step + 1));
            current_values.push((value % mask, step + 1));
        } else {
            current_values.push((value * 2024, step + 1));
        }
    }

    count
}

fn main() -> Result<()> {
    let values: Vec<usize> = stdin()
        .lock()
        .lines()
        .map(|line| {
            line.map(|line: String| {
                line.split_whitespace()
                    .map(|value| value.parse::<usize>())
                    .collect::<Result<Vec<usize>, ParseIntError>>()
            })
        })
        .next()
        .ok_or_else(|| Error::msg("empty stdin"))???;

    let mut mem = Memoizer {
        memory: HashMap::new(),
    };
    let reacheable_from_zero = explore_from_zero();
    println!("{:?}", reacheable_from_zero);

    let total_count: usize = values
        .iter()
        .map(|value| mem.count_after_steps(*value, 25))
        .sum();

    println!("{:?}", total_count);

    let total_count: usize = values
        .iter()
        .map(|value| mem.count_after_steps(*value, 75))
        .sum();
    println!("{:?}", total_count);

    Ok(())
}
