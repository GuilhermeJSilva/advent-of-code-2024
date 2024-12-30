use std::io::{stdin, BufRead};

const DIRECTIONS: [(i64, i64); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];
const MAS_DIRECTIONS: [(i64, i64); 2] = [(1, 1), (-1, -1)];
const MAS_DIRECTIONS_2: [(i64, i64); 2] = [(1, -1), (-1, 1)];

fn matches_char(lines: &Vec<Vec<u8>>, row: &i64, col: &i64, target: &u8) -> bool {
    if let Ok(row) = TryInto::<usize>::try_into(*row) {
        if let Ok(col) = TryInto::<usize>::try_into(*col) {
            return lines
                .get(row)
                .map(|row: &Vec<u8>| {
                    row.get(col)
                        .map(|char: &u8| char.eq(target))
                        .unwrap_or(false)
                })
                .unwrap_or(false);
        }
    }
    false
}

fn search_direction(
    lines: &Vec<Vec<u8>>,
    search_query: &str,
    initial_row: i64,
    initial_col: i64,
    row_delta: i64,
    col_delta: i64,
    offset: i64,
) -> bool {
    search_query
        .bytes()
        .enumerate()
        .map(|(step, value)| (step.try_into().unwrap(), value))
        .map(|(step, value): (i64, u8)| {
            (
                initial_row + row_delta * (step - offset),
                initial_col + col_delta * (step - offset),
                value,
            )
        })
        .find(|(row, col, value)| !matches_char(lines, &row, &col, &value))
        .is_none()
}

fn main() {
    let search_query = "XMAS";

    let lines: Vec<Vec<u8>> = stdin()
        .lock()
        .lines()
        .map(|line| line.expect("read stdin").bytes().collect())
        .collect();

    let n_lines: i64 = lines.len().try_into().unwrap();
    let line_size: i64 = lines
        .first()
        .expect("at least one line")
        .len()
        .try_into()
        .unwrap();
    let xmas_count: usize = (0..n_lines)
        .map(|initial_row| {
            (0..line_size)
                .map(|initial_col| {
                    DIRECTIONS
                        .iter()
                        .filter(|(row_delta, col_delta)| {
                            search_direction(
                                &lines,
                                search_query,
                                initial_row,
                                initial_col,
                                *row_delta,
                                *col_delta,
                                0,
                            )
                        })
                        .count()
                })
                .sum::<usize>()
        })
        .sum();
    println!("{xmas_count}");

    let search_query = "MAS";
    let n_lines: i64 = lines.len().try_into().unwrap();
    let line_size: i64 = lines
        .first()
        .expect("at least one line")
        .len()
        .try_into()
        .unwrap();
    let xmas_count: usize = (0..n_lines)
        .map(|initial_row| {
            (0..line_size)
                .filter(|initial_col| {
                    MAS_DIRECTIONS
                        .iter()
                        .filter(|(row_delta, col_delta)| {
                            search_direction(
                                &lines,
                                search_query,
                                initial_row,
                                *initial_col,
                                *row_delta,
                                *col_delta,
                                1,
                            )
                        })
                        .next()
                        .is_some()
                        && MAS_DIRECTIONS_2
                            .iter()
                            .filter(|(row_delta, col_delta)| {
                                search_direction(
                                    &lines,
                                    search_query,
                                    initial_row,
                                    *initial_col,
                                    *row_delta,
                                    *col_delta,
                                    1,
                                )
                            })
                            .next()
                            .is_some()
                })
                .count()
        })
        .sum();
    println!("{xmas_count}");
}
