use std::collections::{HashMap, HashSet};

use crate::common;

pub fn solve() {
    println!("Day 1");
    let read_lines = common::read_file("src/day1/input.txt");
    println!("Part 1: {}", do_solve(&read_lines, part1_parse));
    println!("Part 2: {}", do_solve(&read_lines, part2_parse));
}

fn do_solve(lines: &[String], parser: fn(&[String]) -> Vec<Vec<i32>>) -> i32 {
    let parsed: Vec<Vec<i32>> = parser(lines);
    parsed.iter().map(|sequence| (sequence.first().unwrap(), sequence.last().unwrap_or(sequence.first().unwrap())))
        .fold(0, |acc, (a, b)| acc + (10 * a) + b)
}

fn part1_parse(lines: &[String]) -> Vec<Vec<i32>> {
    lines.iter().map(|line| {
        line.chars().filter(|c| c.is_ascii_digit()).map(|digit| digit.to_digit(10).unwrap() as i32).collect()
    }).collect()
}

fn part2_parse(lines: &[String]) -> Vec<Vec<i32>> {
    let digits: HashMap<String, i32> = HashMap::from([
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ]);
    let keys: HashSet<String> = digits.keys().map(|k| k.to_string()).collect();

    lines.iter().map(|line| {
        let mut found_digits: Vec<i32> = vec![];
        let mut search = String::new();
        for c in line.chars() {
            search.push(c);

            let found = keys.iter().find(|key: &&String| search.ends_with(*key));
            if search.chars().last().unwrap().is_ascii_digit() {
                found_digits.push(c.to_string().parse::<i32>().unwrap());
            } else if found.is_some() {
                found_digits.push(digits.get(found.unwrap()).unwrap().to_owned());
            }
        }

        found_digits
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_part1_correct() {
        let read_lines = common::read_file("src/day1/test1.txt");

        assert_eq!(do_solve(&read_lines, part1_parse), 142);
    }

    #[test]
    fn is_part2_correct() {
        let read_lines = common::read_file("src/day1/test2.txt");

        assert_eq!(do_solve(&read_lines, part2_parse), 281);
    }
}