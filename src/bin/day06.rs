use std::io::{stdin, BufRead};

const OFFSETS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn walk_the_map(
    map: &Vec<Vec<u8>>,
    mask: &mut Vec<Vec<u8>>,
    row: usize,
    col: usize,
    direction: usize,
) -> bool {
    let mut left_the_map = false;
    let mut row = row;
    let mut col = col;
    let mut direction = direction;
    while mask
        .get(row)
        .expect("non-exitent row")
        .get(col)
        .expect("non-existent col")
        & (1 << direction)
        == 0
    {
        *mask
            .get_mut(row)
            .expect("non-exitent row")
            .get_mut(col)
            .expect("non-existent col") |= 1 << direction;

        if let Ok(next_row) = TryInto::<usize>::try_into(
            TryInto::<i64>::try_into(row).expect("usize must fit usize") + OFFSETS[direction].0,
        ) {
            if let Ok(next_col) = TryInto::<usize>::try_into(
                TryInto::<i64>::try_into(col).expect("usize must fit usize") + OFFSETS[direction].1,
            ) {
                if let Some(map_row) = map.get(next_row) {
                    if let Some(next) = map_row.get(next_col) {
                        if *next == 46 {
                            row = next_row;
                            col = next_col;
                        } else {
                            direction = (direction + 1) % 4;
                        }
                        continue;
                    }
                }
            }
        }
        left_the_map = true;
        break;
    }
    left_the_map
}

fn main() {
    let mut map: Vec<Vec<u8>> = stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().bytes().collect())
        .collect();

    let n_rows = map.len();
    let n_cols = map.first().unwrap().len();

    let direction = 0;
    let (start_row, start_col): (usize, usize) = map
        .iter()
        .enumerate()
        .map(|(row, row_values)| {
            (
                row,
                row_values
                    .iter()
                    .enumerate()
                    .filter(|(_col, value)| value.eq(&&94))
                    .map(|(col, _value)| col)
                    .next(),
            )
        })
        .filter(|(_row, maybe_col)| maybe_col.is_some())
        .map(|(row, maybe_col)| (row, maybe_col.unwrap()))
        .next()
        .expect("starting position not found");

    *map.get_mut(start_row)
        .expect("non-exitent row")
        .get_mut(start_col)
        .expect("non-existent col") = 46;

    let mut mask: Vec<Vec<u8>> = (0..n_rows).map(|_row| vec![0u8; n_cols]).collect();
    walk_the_map(&map, &mut mask, start_row, start_col, direction);

    let count_visited: usize = mask
        .iter()
        .map(|row| row.iter().filter(|value| **value != 0).count())
        .sum();
    println!("{:?}", count_visited);

    let count_loops: usize = mask
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter(|(_col_idx, value)| **value != 0)
                .map(move |(col_idx, _value)| (row_idx, col_idx))
        })
        .filter(|(row, col): &(usize, usize)| {
            *map.get_mut(*row)
                .expect("non-exitent row")
                .get_mut(*col)
                .expect("non-existent col") = 35;
            let mut mask: Vec<Vec<u8>> = (0..n_rows).map(|_row| vec![0u8; n_cols]).collect();
            let loops = !walk_the_map(&map, &mut mask, start_row, start_col, direction);
            *map.get_mut(*row)
                .expect("non-exitent row")
                .get_mut(*col)
                .expect("non-existent col") = 46;
            loops
        })
        .count();
    println!("{:?}", count_loops);
}
