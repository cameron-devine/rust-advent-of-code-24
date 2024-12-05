advent_of_code::solution!(4);

const XMAS: &str = "XMAS";
const MAS: &str = "MAS";

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    UpLeft,
    UpRight,
    Down,
    DownLeft,
    DownRight,
    Left,
    Right,
    XDownRight, // path for searching the down left to right for the X mas
    XUpRight,   // path for searching the up left to right for the X mas
}

impl Direction {
    fn get_indices(&self, row: isize, col: isize, word_len: isize) -> Vec<(isize, isize)> {
        let word_len = word_len;
        match self {
            Direction::Up => (0..word_len).map(|i| (row - i, col)).collect(),
            Direction::UpLeft => (0..word_len).map(|i| (row - i, col - i)).collect(),
            Direction::UpRight => (0..word_len).map(|i| (row - i, col + i)).collect(),
            Direction::Down => (0..word_len).map(|i| (row + i, col)).collect(),
            Direction::DownLeft => (0..word_len).map(|i| (row + i, col - i)).collect(),
            Direction::DownRight => (0..word_len).map(|i| (row + i, col + i)).collect(),
            Direction::Left => (0..word_len).map(|i| (row, col - i)).collect(),
            Direction::Right => (0..word_len).map(|i| (row, col + i)).collect(),
            Direction::XDownRight => (0..word_len).map(|i| (row - 1 + i, col - 1 + i)).collect(),
            Direction::XUpRight => (0..word_len).map(|i| (row + 1 - i, col - 1 + i)).collect(),
        }
    }
}
/// Search for amount of occurrences from a single point
fn search_board_at_point(board: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let directions = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
        Direction::UpLeft,
        Direction::UpRight,
        Direction::DownLeft,
        Direction::DownRight,
    ];

    directions
        .iter()
        .map(|d| {
            d.get_indices(i as isize, j as isize, XMAS.len() as isize)
                .iter()
                .map(|&indexes| match indexes {
                    (row, column) if !row.is_negative() && !column.is_negative() => {
                        board.get(row as usize)?.get(column as usize)
                    }
                    _ => None,
                })
                .collect::<Vec<Option<&char>>>()
        })
        .filter(|letters| {
            letters.iter().all(Option::is_some)
                && letters
                    .into_iter()
                    .flatten()
                    .map(|c| **c)
                    .collect::<String>()
                    == XMAS
        })
        .count() as u32
}

fn is_x_mas_at_point(board: &Vec<Vec<char>>, i: usize, j: usize) -> Option<bool> {
    let directions = vec![Direction::XUpRight, Direction::XDownRight];

    let c = directions
        .iter()
        .map(|d| {
            d.get_indices(i as isize, j as isize, MAS.len() as isize)
                .iter()
                .map(|&indexes| match indexes {
                    (row, column) if !row.is_negative() && !column.is_negative() => {
                        board.get(row as usize)?.get(column as usize)
                    }
                    _ => None,
                })
                .collect::<Vec<Option<&char>>>()
        })
        .filter(|letters| {
            letters.iter().all(Option::is_some) &&
                (letters
                .into_iter()
                .flatten()
                .map(|c| **c)
                .collect::<String>()
                == MAS || letters.into_iter().flatten().rev().map(|c| **c).collect::<String>() == MAS)
        })
        .count() == 2;
    match c {
        true => Some(true),
        false => None
    }
}

fn count_in_board(board: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    for (i, v) in board.iter().enumerate() {
        for (j, c) in v.iter().enumerate() {
            // Only need to search if its the first letter!
            if *c == XMAS.chars().nth(0).unwrap() {
                count += search_board_at_point(&board, i, j);
            }
        }
    }
    count
}

fn count_x_in_board(board: &Vec<Vec<char>>) -> u32 {
    board
        .iter()
        .enumerate()
        .map(|(i, v)| {
            v.iter()
                .enumerate()
                .filter_map(|(j, c)| match c {
                    'A' => is_x_mas_at_point(board, i, j),
                    _ => None,
                })
                .collect::<Vec<bool>>()
                .len() as u32
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>() as u32
}
pub fn part_one(input: &str) -> Option<u32> {
    let ws: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let answer = count_in_board(&ws);
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let ws: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let answer = count_x_in_board(&ws);
    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
