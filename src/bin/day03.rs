use std::io::{stdin, Read};

use regex::Regex;

fn main() {
    let mut code = String::new();
    stdin()
        .lock()
        .read_to_string(&mut code)
        .expect("read string");
    let mul_regex = Regex::new(r"mul\(([[:digit:]]{1,3}),([[:digit:]]{1,3})\)")
        .expect("static regex must compile");
    let instruction_res: i64 = mul_regex
        .captures_iter(&code)
        .map(|c| c.extract())
        .map(|(_, [lhs, rhs])| {
            lhs.parse::<i64>().expect("lhs must be i64")
                * rhs.parse::<i64>().expect("rhs must be i64")
        })
        .sum();
    println!("{instruction_res}");
    let mul_regex =
        Regex::new(r"(mul\(([[:digit:]]{1,3}),([[:digit:]]{1,3})\))|(don't\(\))|(do\(\))")
            .expect("static regex must compile");
    let mut instruction_res: i64 = 0;
    let mut active: bool = true;
    for capture in mul_regex.captures_iter(&code) {
        if let Some(matchs) = capture.get(0) {
            match matchs.as_str() {
                "don't()" => active = false,
                "do()" => active = true,
                _ => {
                    if active {
                        let lhs = capture
                            .get(2)
                            .expect("lhs - mul always has 3 groups")
                            .as_str();
                        let rhs = capture
                            .get(3)
                            .expect("rhs - mul always has 3 groups")
                            .as_str();
                        instruction_res += lhs.parse::<i64>().expect("lhs must be i64")
                            * rhs.parse::<i64>().expect("rhs must be i64")
                    }
                }
            }
        }
    }
    println!("{instruction_res}");
}
