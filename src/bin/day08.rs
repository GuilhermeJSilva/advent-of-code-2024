use std::{i64, io::stdin, usize};

use anyhow::Result;

const EMPTY_VALUE: u8 = b'.';

fn main() -> Result<()> {
    let values: Vec<Vec<u8>> = stdin()
        .lines()
        .map(|line| line.map(|line| line.bytes().collect::<Vec<u8>>()))
        .collect::<Result<_, std::io::Error>>()?;

    let n_rows: i64 = values.len().try_into().unwrap();
    let n_cols: i64 = values[0].len().try_into().unwrap();

    let mut antenas: Vec<Vec<(i64, i64)>> = vec![Vec::new(); u8::MAX as usize];
    values
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter(|(_col_idx, value)| **value != EMPTY_VALUE)
                .map(move |(col_idx, value)| (*value, row_idx, col_idx))
        })
        .for_each(|(value, row_idx, col_idx)| {
            antenas
                .get_mut(value as usize)
                .expect("antenas can fit the entire u8 range")
                .push((row_idx.try_into().unwrap(), col_idx.try_into().unwrap()));
        });

    let mut anomalies: Vec<(i64, i64)> = antenas
        .iter()
        .flat_map(|freq_antenas| {
            freq_antenas
                .iter()
                .flat_map(|antena| {
                    freq_antenas
                        .iter()
                        .map(move |antena_rhs| (antena, antena_rhs))
                })
                .filter(|(lhs, rhs)| lhs != rhs)
                .flat_map(|(lhs, rhs)| {
                    let row_diff = lhs.0 - rhs.0;
                    let col_diff = lhs.1 - rhs.1;
                    let plus = (lhs.0 + row_diff, lhs.1 + col_diff);
                    let minus = (rhs.0 - row_diff, rhs.1 - col_diff);
                    vec![plus, minus]
                })
        })
        .filter(|(row, col)| *row >= 0 && *row < n_rows && *col >= 0 && *col < n_cols)
        .collect();
    anomalies.sort();
    anomalies.dedup();
    println!("{:?}", anomalies.len());

    let mut anomalies: Vec<(i64, i64)> = antenas
        .iter()
        .flat_map(|freq_antenas| {
            freq_antenas
                .iter()
                .flat_map(|antena| {
                    freq_antenas
                        .iter()
                        .map(move |antena_rhs| (antena, antena_rhs))
                })
                .filter(|(lhs, rhs)| lhs != rhs)
                .flat_map(|(lhs, rhs)| {
                    let row_diff = lhs.0 - rhs.0;
                    let col_diff = lhs.1 - rhs.1;
                    let mut anomalies = Vec::new();
                    let mut multiplier = 0;
                    loop {
                        let mut added = false;
                        let plus = (lhs.0 + multiplier * row_diff, lhs.1 + multiplier * col_diff);
                        if plus.0 >= 0 && plus.0 < n_rows && plus.1 >= 0 && plus.1 < n_cols {
                            anomalies.push(plus);
                            added = true;
                        }
                        if !added {
                            break;
                        }
                        multiplier += 1;
                    }
                    let mut multiplier = 0;
                    loop {
                        let mut added = false;
                        let minus = (rhs.0 - multiplier * row_diff, rhs.1 - multiplier * col_diff);
                        if minus.0 >= 0 && minus.0 < n_rows && minus.1 >= 0 && minus.1 < n_cols {
                            anomalies.push(minus);
                            added = true;
                        }
                        if !added {
                            break;
                        }
                        multiplier += 1;
                    }
                    anomalies
                })
        })
        .collect();
    anomalies.sort();
    anomalies.dedup();
    println!("{:?}", anomalies.len());

    Ok(())
}
