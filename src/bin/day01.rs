use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn main() {
    let mut right_list: Vec<i64> = Vec::new();
    let mut left_list: Vec<i64> = Vec::new();
    for line in stdin().lock().lines() {
        if let Some((left, right)) = line.expect("successful read").split_once("   ") {
            let left_n: i64 = left.parse().expect("a number");
            left_list.push(left_n);
            let right_n: i64 = right.parse().expect("a number");
            right_list.push(right_n);
        }
    }

    right_list.sort();
    left_list.sort();

    let sum_of_diffs = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| (left - right).abs())
        .sum::<i64>();

    println!("{sum_of_diffs}");

    let mut right_count: HashMap<i64, i64> = HashMap::new();
    for right in right_list {
        if right_count.contains_key(&right) {
            *right_count.get_mut(&right).unwrap() += 1;
        } else {
            right_count.insert(right, 1);
        }
    }
    let similarity: i64 = left_list
        .iter()
        .map(|left| right_count.get(left).copied().unwrap_or(0) * left)
        .sum();
    println!("{similarity}");
    ()
}
