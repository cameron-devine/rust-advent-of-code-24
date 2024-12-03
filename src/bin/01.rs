use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut side1 = vec![];
    let mut side2 = vec![];

    input.lines().for_each(|line| {
        let mut split = line.split_whitespace();
        side1.push(split.next().unwrap().parse::<i32>().unwrap());
        side2.push(split.next().unwrap().parse::<i32>().unwrap());
    });
    side1.sort_unstable();
    side2.sort_unstable();

    let mut answer = 0;
    side1.iter().zip(side2.iter()).for_each(|(loc1, loc2)| {
        answer += loc1.abs_diff(*loc2);
    });

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut side1 = vec![];
    let mut side2 = HashMap::new();

    input.lines().for_each(|line| {
        let mut split = line.split_whitespace();
        side1.push(split.next().unwrap().parse::<u32>().unwrap());
        let right_entry = split.next().unwrap().parse::<u32>().unwrap();
        *side2.entry(right_entry).or_insert(0) += 1;
    });

    let mut answer: u32 = 0;
    side1.iter().for_each(|loc| {
        answer += loc * side2.get(loc).unwrap_or(&0);
    });

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
