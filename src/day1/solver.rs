use std::{collections::{HashMap, HashSet}};

use crate::common;

pub fn solve() {
    println!("Day 1");
    let read_lines = common::read_file("src/day1/input.txt");
    println!("Part 1: {}", do_solve(&read_lines, part1_parse));
    println!("Part 2: {}", do_solve(&read_lines, part2_parse));
}

fn do_solve(lines: &Vec<String>, parser: fn(&Vec<String>) -> Vec<Vec<i32>>) -> i32 {
    let parsed: Vec<Vec<i32>> = parser(&lines);
    parsed.iter().map(|sequence| (sequence.first().unwrap(), sequence.last().unwrap_or(sequence.first().unwrap())))
        .fold(0, |acc, (a, b)| acc + (10 * a) + b)
}

fn part1_parse(lines: &Vec<String>) -> Vec<Vec<i32>> {
    lines.iter().map(|line| {
        line.chars().filter(|c| c.is_ascii_digit()).map(|digit| digit.to_digit(10).unwrap() as i32).collect()
    }).collect()
}

fn part2_parse(lines: &Vec<String>) -> Vec<Vec<i32>> {
    lines.iter().map(|line| deal_with_line(line.to_string())).collect()
}

fn deal_with_line(line: String) -> Vec<i32> {
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

    let mut found_digits: Vec<i32> = vec![];
    let mut search = String::new();
    for c in line.chars() {
        search.push(c);

        let found = ends_with_set_member(&search, &keys);
        if search.chars().last().unwrap().is_ascii_digit() {
            found_digits.push(c.to_string().parse::<i32>().unwrap());
        } else if found.is_some() {
            found_digits.push(digits.get(found.unwrap()).unwrap().clone());
        }
    }

    found_digits
}

fn ends_with_set_member<'a>(input: &'a String, set: &'a HashSet<String>) -> Option<&'a String> {
    set.iter().find(|key: &&String| input.ends_with(*key))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let read_lines = common::read_file("src/day1/test1.txt");

        assert_eq!(do_solve(&read_lines, part1_parse), 142);
    }

    #[test]
    fn part2() {
        let read_lines = common::read_file("src/day1/test2.txt");

        assert_eq!(do_solve(&read_lines, part2_parse), 281);
    }
}