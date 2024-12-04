use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let pairs: Vec<(i32, i32)> = input
        .lines()
        .filter_map(|line| {
            let mut split = line.split_whitespace();
            Some((
                split.next()?.parse::<i32>().ok()?,
                split.next()?.parse::<i32>().ok()?,
            ))
        })
        .collect();

    // Separate and sort
    let (mut side1, mut side2): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
    side1.sort_unstable();
    side2.sort_unstable();

    let sum = side1
        .iter()
        .zip(side2.iter())
        .map(|(&a, &b)| a.abs_diff(b))
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut side1 = vec![];
    let mut side2 = HashMap::new();

    input.lines().for_each(|line| {
        let mut split = line.split_whitespace();
        if let (Some(left), Some(right)) = (split.next(), split.next()) {
            if let (Ok(left_parsed), Ok(right_parsed)) = (left.parse::<u32>(), right.parse::<u32>()) {
                side1.push(left_parsed);
                *side2.entry(right_parsed).or_insert(0) += 1;
            }
        }
    });

    let answer = side1
        .iter()
        .map(|loc| loc * side2.get(loc).unwrap_or(&0))
        .sum();

    Some(answer)
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
