use core::panic;
use std::{collections::HashMap, io::stdin};

fn topo_order(rules: &Vec<(u64, u64)>) -> Vec<u64> {
    let mut adjacency_list: HashMap<u64, Vec<u64>> = HashMap::new();
    for (before, after) in rules {
        let after_adj = adjacency_list.entry(*after).or_insert_with(|| Vec::new());
        after_adj.push(*before);
        adjacency_list.entry(*before).or_insert_with(|| Vec::new());
    }

    let mut topo_order: Vec<u64> = Vec::new();
    while !adjacency_list.is_empty() {
        let without_before: Vec<u64> = adjacency_list
            .iter()
            .filter(|(_num, requirements)| requirements.is_empty())
            .map(|(num, _requirements)| *num)
            .collect();
        if without_before.is_empty() {
            println!("{:?}", adjacency_list);
            panic!("can not find topo order");
        }
        if without_before.len() > 1 {
            println!("{:?}", without_before);
            panic!("multiple topo orders");
        }
        for to_remove in &without_before {
            adjacency_list.remove(to_remove);
        }
        for (_num, requirements) in adjacency_list.iter_mut() {
            for to_remove in &without_before {
                requirements.retain_mut(|value| value != to_remove)
            }
        }

        topo_order.push(*without_before.get(0).unwrap());
    }
    return topo_order;
}

fn main() {
    let rules: Vec<(u64, u64)> = stdin()
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.eq(""))
        .map(|line| {
            let (lhs, rhs) = line.split_once('|').unwrap();
            (lhs.parse().unwrap(), rhs.parse().unwrap())
        })
        .collect();

    let updates: Vec<Vec<u64>> = stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.split(',')
                .map(|value| value.parse().unwrap())
                .collect()
        })
        .collect();

    let mut middle_sum = 0;
    let mut middle_incorrect = 0;
    for update in &updates {
        let applicable_rules = rules
            .iter()
            .filter(|(before, after)| update.contains(before) && update.contains(after))
            .map(|(before, after)| (*before, *after))
            .collect();
        let topo = topo_order(&applicable_rules);
        if topo
            .iter()
            .zip(update.iter())
            .filter(|(t, u)| t == u)
            .count()
            == update.len()
        {
            let middle = update.len() / 2;
            middle_sum += update.get(middle).unwrap();
        } else {
            let middle = topo.len() / 2;
            middle_incorrect += topo.get(middle).unwrap();
        }
    }

    println!("{:?}", middle_sum);
    println!("{:?}", middle_incorrect);
}
