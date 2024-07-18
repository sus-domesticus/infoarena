// https://infoarena.ro/problema/strmatch

use std::fs;
use std::fs::read_to_string;

fn parse_input() -> (Vec<u8>, Vec<u8>) {
    let text = read_to_string("strmatch.in").unwrap();
    let break_index = text.find('\n').unwrap();

    let mut tmp = text;
    let second_word = tmp.split_off(break_index + 1);
    tmp.pop();
    let first_word = tmp;

    (first_word.into_bytes(), second_word.into_bytes())
}

fn get_lps_array(word: &[u8]) -> Vec<usize> {
    let mut lps = vec![0usize; word.len()];
    let mut i = 0usize;
    let mut j = 1usize;
    while j < word.len() {
        if word[i] == word[j] {
            lps[j] = lps[i] + 1;
            i += 1;
            j += 1;
        } else if i != 0 {
            i = lps[i - 1];
        } else {
            j += 1;
        }
    }
    lps
}

fn main() {
    let (a, b) = parse_input();

    let mut positions: Vec<usize> = Vec::new();
    let lps = get_lps_array(&a);
    let mut i = 0usize;
    let mut j = 0usize;
    while j < b.len() {
        if i < a.len() && a[i] == b[j] {
            if (i + 1) == a.len() && (positions.len() < 1000) {
                positions.push(j - i);
            }
            i += 1;
            j += 1;
        } else if i != 0 {
            i = lps[i - 1];
        } else {
            j += 1;
        }
    }
    let result_text = format!(
        "{}\n{}",
        positions.len(),
        positions
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
    fs::write("strmatch.out", result_text).unwrap();
}
