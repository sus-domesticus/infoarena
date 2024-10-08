// https://www.infoarena.ro/problema/comun

use std::{collections::HashSet, fs};

fn parse_input() -> Vec<u32> {
    let text = fs::read_to_string("comun.in").unwrap();
    let break_index = text.find('\n').unwrap();

    let mut tmp = text;
    let numbers = tmp.split_off(break_index + 1);
    numbers
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect()
}

fn calc_gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let c = a % b;
        a = b;
        b = c;
    }
    a
}

fn main() {
    let numbers = parse_input();

    let mut set: HashSet<u32> = HashSet::new();
    let mut tmp_max = numbers.first().unwrap();
    for value in &numbers {
        tmp_max = tmp_max.max(value);
        set.insert(*value);
    }

    let max = *tmp_max;
    let mut result = Vec::new();
    for number in numbers {
        let mut multiple = number * 2;
        let mut gcd = 0;
        while multiple <= max {
            if set.contains(&multiple) {
                gcd = calc_gcd(gcd, multiple / number);
            }
            if gcd == 1 {
                break;
            }
            multiple += number;
        }
        if gcd != 1 {
            result.push(number);
        }
    }

    let result_text = format!(
        "{}\n{}",
        result.len(),
        result
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
    fs::write("comun.out", result_text).unwrap();
}
