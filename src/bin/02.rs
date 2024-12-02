use itertools::Itertools;

advent_of_code::solution!(2);

enum Direction {
    Ascending,
    Descending,
}

pub fn part_one(input: &str) -> Option<u32> {
    let result: Vec<bool> = input.lines()
        .map(|line| {
            let nums: Vec<u32> = line.split_ascii_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            let dir = if nums[0] > nums[1] { Direction::Descending } else { Direction::Ascending };
            let instances = nums.iter().tuple_windows().all(|(a, b)| {
                let calc = a.abs_diff(*b);
                if calc == 0 {
                    return false;
                }
                let ret = match dir {
                    Direction::Ascending => *a < *b && calc >= 1 && calc <= 3,
                    Direction::Descending => *a > *b && calc >= 1 && calc <= 3,
                };
                ret
            });
            instances
        })
        .collect();
    Some(result.iter().filter(|&&num| num == true).count() as u32)
}

// Time to rethink implementation...
pub fn part_two(input: &str) -> Option<u32> {
    let mut counter = 0;
    for line in input.lines() {
        let nums: Vec<u32> = line.split_ascii_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect();
        if is_safe(&nums) {
            counter += 1;
        } else {
            for i in 0..nums.len() {
                let new_nums = copy_without_index(&nums, &i);
                if is_safe(&new_nums) {
                    counter += 1;
                    break;
                }
            }
        }
    }
    Some(counter)
}

fn is_safe(vec: &Vec<u32>) -> bool {
    let dir = if vec[0] > vec[1] { Direction::Descending } else { Direction::Ascending };
    let instances = vec.iter().tuple_windows().all(|(a, b)| {
        let calc = a.abs_diff(*b);
        if calc == 0 {
            return false;
        }
        let ret = match dir {
            Direction::Ascending => *a < *b && calc >= 1 && calc <= 3,
            Direction::Descending => *a > *b && calc >= 1 && calc <= 3,
        };
        ret
    });
    instances
}

fn copy_without_index(vec: &Vec<u32>, i: &usize) -> Vec<u32> {
    let mut new_vec = Vec::with_capacity(vec.len() - 1);
    for (index, value) in vec.iter().enumerate() {
        if index != *i {
            new_vec.push(*value);
        }
    }
    new_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
