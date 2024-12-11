use anyhow::Result;

use std::io::{stdin, BufRead};

const ZERO: u8 = '0' as u8;

#[derive(Copy, Clone, Debug)]
struct File {
    size: usize,
    value: usize,
    moved: bool,
}

#[derive(Clone, Debug)]
struct Block {
    capacity: usize,
    free: usize,
    occuppied: Vec<File>,
}

fn per_block_compaction(block_sizes: &Vec<usize>) -> usize {
    let n_blocks = block_sizes.len();
    let mut from_start_idx = 0;
    let mut start_block_value = 0;
    let mut from_end_idx = n_blocks - 1;
    let mut end_block_value = from_end_idx / 2usize;
    let mut end_block_size = block_sizes[from_end_idx];
    let mut position = 0;
    let mut checksum = 0;

    // let mut compacted = String::new();
    while from_start_idx < from_end_idx {
        let filled_block_size = block_sizes[from_start_idx];
        for block_position in 0..filled_block_size {
            checksum += (block_position + position) * start_block_value;
            // compacted += &start_block_value.to_string();
        }
        position += filled_block_size;
        start_block_value += 1;
        from_start_idx += 1;
        if from_start_idx >= from_end_idx {
            break;
        }
        let to_be_filled_block_size = block_sizes[from_start_idx];
        for block_position in 0..to_be_filled_block_size {
            if end_block_size == 0 {
                from_end_idx -= 2;
                if from_start_idx >= from_end_idx {
                    break;
                }
                end_block_size = block_sizes[from_end_idx];
                end_block_value -= 1;
            }
            checksum += (block_position + position) * end_block_value;
            // compacted += &end_block_value.to_string();
            end_block_size -= 1;
        }
        position += to_be_filled_block_size;
        from_start_idx += 1;
    }
    for block_position in 0..end_block_size {
        checksum += (block_position + position) * end_block_value;
        // compacted += &end_block_value.to_string();
    }
    // println!("{:?}", compacted);
    return checksum;
}

fn block_checksum(blocks: &Vec<Block>) -> usize {
    let mut position = 0;
    let mut checksum = 0;
    // let mut compacted = String::new();
    for block in blocks {
        for file in &block.occuppied {
            for block_position in 0..file.size {
                checksum += (block_position + position) * file.value;
                // compacted += &file.value.to_string();
            }
            position += file.size;
        }
        // compacted += &".".repeat(block.free);
        position += block.free;
    }
    // println!("{:?}", compacted);
    return checksum;
}

fn main() -> Result<()> {
    let first_line = stdin()
        .lock()
        .lines()
        .next()
        .ok_or(anyhow::Error::msg("no lines in stdin"))??;
    let block_sizes: Vec<usize> = first_line
        .bytes()
        .map(|byte| (byte - ZERO) as usize)
        .collect();
    let n_blocks = block_sizes.len();
    let checksum = per_block_compaction(&block_sizes);
    println!("{:?}", checksum);

    let mut blocks: Vec<Block> = block_sizes
        .iter()
        .enumerate()
        .map(|(idx, size)| {
            if idx % 2 == 0 {
                Block {
                    capacity: *size,
                    free: 0,
                    occuppied: vec![File {
                        size: *size,
                        value: idx / 2,
                        moved: false,
                    }],
                }
            } else {
                Block {
                    capacity: *size,
                    free: *size,
                    occuppied: vec![],
                }
            }
        })
        .collect();

    for idx_to_relocate in (0..n_blocks).rev() {
        // let _checksum = block_checksum(&blocks);
        if blocks[idx_to_relocate].occuppied.len() != 1 {
            continue;
        }
        if let Some(file_to_relocate) = blocks[idx_to_relocate]
            .occuppied
            .iter()
            .filter(|file| !file.moved)
            .map(|file| *file)
            .next()
        {
            for target_block in 0..idx_to_relocate {
                if blocks[target_block].free >= file_to_relocate.size {
                    blocks[target_block].occuppied.push(File {
                        size: file_to_relocate.size,
                        value: file_to_relocate.value,
                        moved: true,
                    });
                    blocks[target_block].free -= file_to_relocate.size;
                    blocks[idx_to_relocate].occuppied.pop();
                    blocks[idx_to_relocate].free = blocks[idx_to_relocate].capacity;
                    break;
                }
            }
        }
    }

    let checksum = block_checksum(&blocks);
    println!("{:?}", checksum);
    Ok(())
}
