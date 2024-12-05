use std::io::stdin;

fn main() {
    let rules: Vec<(u64, u64)> = stdin()
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.eq(""))
        .map(|line| {
            println!("{line}");
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

    println!("{:?}", rules);
    println!("{:?}", updates);
}
