use std::io::{self, stdin};

use anyhow::{anyhow, Result};

#[derive(Clone, Copy, Debug)]
enum Group {
    Assigned(usize),
    Unassigned,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mapping {
    Root,
    Child(usize),
}

#[derive(Clone, Debug)]
struct Grouppings {
    areas: Vec<usize>,
    perimeters: Vec<usize>,
    faces: Vec<usize>,
}

impl Grouppings {
    pub fn from_group_map(map: &Vec<Vec<u8>>) -> anyhow::Result<Self> {
        if map.is_empty() || map[0].is_empty() {
            return Err(anyhow!("map to group must not be empty"));
        }
        let n_rows = map.len();
        let n_cols = map[0].len();

        let mut group_match = vec![vec![Group::Unassigned; n_rows]; n_cols];
        let mut group_to_char = Vec::new();
        let mut mappings = Vec::new();
        let mut areas = Vec::new();
        let mut perimeters = Vec::new();
        let mut faces = Vec::new();

        for row in 0..n_rows {
            for col in 0..n_cols {
                let prev_row_group = if let Some(prev_row) = row.checked_sub(1) {
                    if map[prev_row][col] == map[row][col] {
                        match group_match[prev_row][col] {
                            Group::Assigned(group) => Some(group),
                            Group::Unassigned => panic!("previous is unassigned"),
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                let prev_col_group = if let Some(prev_col) = col.checked_sub(1) {
                    if map[row][prev_col] == map[row][col] {
                        match group_match[row][prev_col] {
                            Group::Assigned(group) => Some(group),
                            Group::Unassigned => panic!("previous is unassigned"),
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                let group: usize = if let Some(prev_row_group) = prev_row_group {
                    if let Some(prev_col_group) = prev_col_group {
                        let col_group = Grouppings::to_group(&mut mappings, prev_col_group);
                        let row_group = Grouppings::to_group(&mut mappings, prev_row_group);
                        if col_group == row_group {
                            col_group
                        } else {
                            let target_group = col_group.min(row_group);
                            let child_group = col_group.max(row_group);
                            mappings[child_group] = Mapping::Child(target_group);
                            areas[target_group] += areas[child_group];
                            perimeters[target_group] += perimeters[child_group];
                            faces[target_group] += faces[child_group];
                            target_group
                        }
                    } else {
                        Grouppings::to_group(&mut mappings, prev_row_group)
                    }
                } else if let Some(prev_col_group) = prev_col_group {
                    Grouppings::to_group(&mut mappings, prev_col_group)
                } else {
                    let new_group = mappings.len();
                    mappings.push(Mapping::Root);
                    group_to_char.push(map[row][col]);
                    areas.push(0);
                    perimeters.push(0);
                    faces.push(0);
                    new_group
                };

                group_match[row][col] = Group::Assigned(group);
                areas[group] += 1;
                perimeters[group] += Grouppings::cell_perimeter(map, row, col);
                faces[group] += Grouppings::cell_faces(map, row, col);
            }
        }

        let areas = areas
            .into_iter()
            .enumerate()
            .filter(|(idx, _area)| mappings[*idx] == Mapping::Root)
            .map(|(_idx, value)| value)
            .collect();
        let perimeters = perimeters
            .into_iter()
            .enumerate()
            .filter(|(idx, _perimeter)| mappings[*idx] == Mapping::Root)
            .map(|(_idx, value)| value)
            .collect();
        let faces = faces
            .into_iter()
            .enumerate()
            .filter(|(idx, _face)| mappings[*idx] == Mapping::Root)
            .map(|(_idx, value)| value)
            .collect();

        Ok(Grouppings {
            areas,
            perimeters,
            faces,
        })
    }

    fn to_group(mappings: &mut Vec<Mapping>, start_group: usize) -> usize {
        let mut group = start_group;
        loop {
            match mappings[group] {
                Mapping::Root => break group,
                Mapping::Child(parent) => group = parent,
            }
        }
    }

    fn cell_perimeter(map: &Vec<Vec<u8>>, row: usize, col: usize) -> usize {
        let mut perimeter = 0;
        if let Some(prev_row) = row.checked_sub(1) {
            if map[prev_row][col] != map[row][col] {
                perimeter += 1;
            }
        } else {
            perimeter += 1;
        }
        if let Some(prev_col) = col.checked_sub(1) {
            if map[row][prev_col] != map[row][col] {
                perimeter += 1;
            }
        } else {
            perimeter += 1;
        }

        if let Some(value) = map[row].get(col + 1) {
            if *value != map[row][col] {
                perimeter += 1;
            }
        } else {
            perimeter += 1;
        }

        if let Some(value) = map.get(row + 1).map(|row_value| row_value[col]) {
            if value != map[row][col] {
                perimeter += 1;
            }
        } else {
            perimeter += 1;
        }
        perimeter
    }

    fn cell_faces(map: &Vec<Vec<u8>>, row: usize, col: usize) -> usize {
        let n_cols = map[0].len();
        let n_rows = map.len();
        let mut faces = 0;
        let cell_value = map[row][col];
        // Top face
        if let Some(prev_row) = row.checked_sub(1) {
            if cell_value != map[prev_row][col] {
                if let Some(prev_col) = col.checked_sub(1) {
                    if cell_value != map[row][prev_col] || cell_value == map[prev_row][prev_col] {
                        faces += 1;
                    }
                } else {
                    faces += 1;
                }
            }
        } else {
            if let Some(prev_col) = col.checked_sub(1) {
                if cell_value != map[row][prev_col] {
                    faces += 1;
                }
            } else {
                faces += 1;
            }
        };

        // Left face
        if let Some(prev_col) = col.checked_sub(1) {
            if cell_value != map[row][prev_col] {
                if let Some(prev_row) = row.checked_sub(1) {
                    if cell_value != map[prev_row][col] || cell_value == map[prev_row][prev_col] {
                        faces += 1;
                    }
                } else {
                    faces += 1;
                }
            }
        } else {
            if let Some(prev_row) = row.checked_sub(1) {
                if cell_value != map[prev_row][col] {
                    faces += 1;
                }
            } else {
                faces += 1;
            }
        };

        // Right face
        let next_col = col.checked_add(1);
        if next_col.is_some() && next_col.unwrap() < n_cols {
            let next_col = next_col.unwrap();
            if cell_value != map[row][next_col] {
                if let Some(prev_row) = row.checked_sub(1) {
                    if cell_value != map[prev_row][col] || cell_value == map[prev_row][next_col] {
                        faces += 1;
                    }
                } else {
                    faces += 1;
                }
            }
        } else {
            if let Some(prev_row) = row.checked_sub(1) {
                if cell_value != map[prev_row][col] {
                    faces += 1;
                }
            } else {
                faces += 1;
            }
        }

        // Bottom face
        let next_row = row.checked_add(1);
        if next_row.is_some() && next_row.unwrap() < n_rows {
            let next_row = next_row.unwrap();
            if cell_value != map[next_row][col] {
                if let Some(prev_col) = col.checked_sub(1) {
                    if cell_value != map[row][prev_col] || cell_value == map[next_row][prev_col] {
                        faces += 1;
                    }
                } else {
                    faces += 1;
                }
            }
        } else {
            if let Some(prev_col) = col.checked_sub(1) {
                if cell_value != map[row][prev_col] {
                    faces += 1;
                }
            } else {
                faces += 1;
            }
        };

        faces
    }
}

fn main() -> anyhow::Result<()> {
    let map: Vec<Vec<u8>> = stdin()
        .lines()
        .map(|line| line.map(|line| line.bytes().collect::<Vec<u8>>()))
        .collect::<Result<_, io::Error>>()?;
    let grouppings = Grouppings::from_group_map(&map)?;
    let price: usize = grouppings
        .areas
        .iter()
        .zip(grouppings.perimeters)
        .map(|(area, perimeter)| area * perimeter)
        .sum();
    println!("{price}");
    let price: usize = grouppings
        .areas
        .iter()
        .zip(grouppings.faces)
        .map(|(area, face)| area * face)
        .sum();
    println!("{price}");
    Ok(())
}
