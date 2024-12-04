advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(([1-9]{1}[0-9]{0,2}),([1-9]{1}[0-9]{0,2})\)").unwrap();
    let mut results = vec![];

    for (_, [val1, val2]) in re.captures_iter(input).map(|c| c.extract()) {
        results.push(val1.parse::<u32>().unwrap() * val2.parse::<u32>().unwrap());
    }
    Some(results.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();
    let mut enabled = true;

    let sum = re.captures_iter(input)
        .filter_map(|cap| {
            if let (Some(x), Some(y)) = (cap.get(1), cap.get(2)) {
                if enabled {
                    let x = x.as_str().parse::<i32>().unwrap();
                    let y = y.as_str().parse::<i32>().unwrap();
                    Some(x * y)
                } else {
                    None
                }
            } else {
                match &cap[0] {
                    "don't()" => enabled = false,
                    "do()" => enabled = true,
                    _ => {}
                }
                None
            }
        })
        .sum::<i32>();
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
