use std::{collections::HashMap, iter::zip};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    let mut result: Vec<u32> = Vec::new();

    for line in input.lines() {
        let arr: Vec<&str> = line.trim().split_whitespace().collect();
        left.push(arr[0].parse().unwrap());
        right.push(arr[1].parse().unwrap());
    }
    left.sort();
    right.sort();

    for (l, r) in zip(left, right) {
        result.push(l.abs_diff(r));
    }

    Some(result.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = Vec::new();
    let mut right: HashMap<u32, u32> = HashMap::new();
    let mut result: Vec<u32> = Vec::new();

    for line in input.lines() {
        let arr: Vec<&str> = line.trim().split_whitespace().collect();
        left.push(arr[0].parse().unwrap());
        let count = right.entry(arr[1].parse().unwrap()).or_insert(0);
        *count += 1;
    }

    for key in left.iter() {
        result.push(key * right.get(key).copied().unwrap_or(0));
    }

    Some(result.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
