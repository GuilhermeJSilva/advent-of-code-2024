use std::io::{stdin, BufRead};

fn safe_level(level: &[i64]) -> bool {
    let diff = level.windows(2usize).all(|window| {
        let diff = (window[0] - window[1]).abs();
        (1..=3).contains(&diff)
    });

    let all_increase = level
        .windows(2usize)
        .map(|window| window[0] - window[1])
        .all(|diff| diff > 0);
    let all_decrease = level
        .windows(2usize)
        .map(|window| window[0] - window[1])
        .all(|diff| diff < 0);
    diff && (all_increase || all_decrease)
}

fn main() {
    let report_levels: Vec<Vec<i64>> = stdin()
        .lock()
        .lines()
        .map(|line| {
            line.expect("reading line")
                .split_whitespace()
                .map(|value| value.parse::<i64>().expect("parsing i64"))
                .collect()
        })
        .collect();

    let safe_levels_count = report_levels
        .iter()
        .filter(|level| safe_level(level))
        .count();
    println!("{safe_levels_count}");
    let safe_levels_count = report_levels
        .iter()
        .filter(|level| {
            safe_level(level)
                || (0..level.len()).any(|to_remove| {
                    let mut with_removed: Vec<i64> = level.to_vec();
                    with_removed.remove(to_remove);
                    safe_level(&with_removed)
                })
        })
        .count();
    println!("{safe_levels_count}");
}
