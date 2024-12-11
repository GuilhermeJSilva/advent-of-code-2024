use anyhow::Result;
use std::io::{stdin, BufRead};

const ZERO: u8 = '0' as u8;

fn count_summits(
    height_map: &Vec<Vec<u8>>,
    position: (usize, usize),
    summits: &mut Vec<(usize, usize)>,
) -> Result<()> {
    let current_height = height_map[position.0][position.1];
    if current_height == 9 {
        summits.push(position);
        return Ok(());
    }
    if let Some(new_row) = position.0.checked_sub(1) {
        if current_height + 1 == height_map[new_row][position.1] {
            count_summits(height_map, (new_row, position.1), summits)?;
        }
    }
    if let Some(new_col) = position.1.checked_sub(1) {
        if current_height + 1 == height_map[position.0][new_col] {
            count_summits(height_map, (position.0, new_col), summits)?;
        }
    }
    if position.0 + 1 < height_map.len() {
        if current_height + 1 == height_map[position.0 + 1][position.1] {
            count_summits(height_map, (position.0 + 1, position.1), summits)?;
        }
    }
    if position.1 + 1 < height_map[0].len() {
        if current_height + 1 == height_map[position.0][position.1 + 1] {
            count_summits(height_map, (position.0, position.1 + 1), summits)?;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let height_map: Vec<Vec<u8>> = stdin()
        .lock()
        .lines()
        .map(|line| {
            line.map(|line| {
                line.bytes()
                    .map(|height| height - ZERO)
                    .collect::<Vec<u8>>()
            })
        })
        .collect::<Result<_, std::io::Error>>()?;

    let mut total_trailhead_score: usize = 0;
    let mut total_trailhead_rating: usize = 0;
    for row_idx in 0..height_map.len() {
        for col_idx in 0..height_map[0].len() {
            if height_map[row_idx][col_idx] == 0 {
                let mut summits = Vec::new();
                count_summits(&height_map, (row_idx, col_idx), &mut summits)?;
                total_trailhead_rating += summits.len();
                summits.sort();
                summits.dedup();
                total_trailhead_score += summits.len();
            }
        }
    }
    println!("{:?}", total_trailhead_score);
    println!("{:?}", total_trailhead_rating);
    Ok(())
}
