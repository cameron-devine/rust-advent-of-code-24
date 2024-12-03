use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let multiplier_regex: Regex = Regex::new(r"mul\((?<m>\d{1,3}),(?<n>\d{1,3})\)").unwrap();
    let sum_of_multiples: Vec<u32> = multiplier_regex
        .captures_iter(input)
        .map(|caps| {
            let m = caps.name("m").unwrap().as_str().parse::<u32>().unwrap();
            let n = caps.name("n").unwrap().as_str().parse::<u32>().unwrap();
            m * n
        })
        .collect();
    let sum: u32 = sum_of_multiples.iter().sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let multiplier_regex: Regex =
        Regex::new(r"mul\((?<m>\d{1,3}),(?<n>\d{1,3})\)|(?<do>do\(\))|(?<dont>don't\(\))").unwrap();
    let all_matches = multiplier_regex.captures_iter(input);

    let mut answer = 0;
    let mut should_multi = true;
    for captures in all_matches {
        if captures.name("do").is_some() {
            should_multi = true;
        } else if captures.name("dont").is_some() {
            should_multi = false;
        } else if captures.name("m").is_some() && captures.name("n").is_some() && should_multi {
            answer += captures.name("m").unwrap().as_str().parse::<u32>().unwrap()
                * captures.name("n").unwrap().as_str().parse::<u32>().unwrap();
        }
    }
    Some(answer)
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
