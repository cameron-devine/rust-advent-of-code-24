use itertools::iproduct;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

advent_of_code::solution!(8);

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Coordinates(isize, isize);
impl Coordinates {
    fn interpolate_from(&self, point: &Coordinates) -> Coordinates {
        let anti_x = point.0 + (point.0 - self.0);
        let anti_y = point.1 + (point.1 - self.1);
        Coordinates(anti_x, anti_y)
    }
}
/// 0,0 -> 1,1 = 1 + 1 - 0, 1 + 1 - 0 (2,2)
/// 1,1 -> 0,0 = 0 + 0 - 1, 0 + 0 - 1 (-1,-1)
#[derive(Debug, Eq, PartialEq)]
struct ParseWorldError;

struct World {
    antennas: HashMap<char, HashSet<Coordinates>>,
    anti_nodes: HashSet<Coordinates>,
    length: isize,
    width: isize,
}

impl World {
    fn new(
        antennas: HashMap<char, HashSet<Coordinates>>,
        anti_nodes: HashSet<Coordinates>,
        length: isize,
        width: isize,
    ) -> World {
        World {
            antennas,
            anti_nodes,
            length,
            width,
        }
    }

    fn is_point_on_grid(&self, point: &Coordinates) -> bool {
        point.0 >= 0 && point.0 < self.width && point.1 >= 0 && point.1 < self.length
    }

    fn interpolate_all_anti_nodes(&mut self) -> () {
        let anti_nodes = self
            .antennas
            .iter()
            .map(|(_key, set)| {
                let mut c_anti_nodes = HashSet::new();
                for (a, b) in iproduct!(set.iter(), set.iter()) {
                    let mut point_a = a.clone();
                    let mut point_b = b.clone();
                    if a == b {
                        c_anti_nodes.insert(point_a);
                    } else {
                        while self.is_point_on_grid(&point_b) {
                            let next_point = point_a.interpolate_from(&point_b);
                            c_anti_nodes.insert(next_point);
                            point_a = point_b;
                            point_b = next_point;
                        }
                    }
                }
                c_anti_nodes
            })
            .flatten()
            .collect::<HashSet<_>>();

        self.anti_nodes = anti_nodes;
    }

    fn anti_node_count(&self) -> u32 {
        self.anti_nodes
            .iter()
            .filter(|node| self.is_point_on_grid(node))
            .count() as u32
    }

}

impl FromStr for World {
    type Err = ParseWorldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut length = 0;
        let mut width = 0;
        let mut antennas = HashMap::new();

        for (i, line) in s.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                width = j + 1;
                match c {
                    '.' => continue,
                    _ => {
                        antennas
                            .entry(c)
                            .or_insert(HashSet::new())
                            .insert(Coordinates(i as isize, j as isize));
                    }
                }
            }
            length = i + 1;
        }
        let anti_nodes = antennas
            .iter()
            .map(|(_key, set)| {
                let mut c_anti_nodes = HashSet::new();
                for (a, b) in iproduct!(set.iter(), set.iter()) {
                    if a != b {
                        c_anti_nodes.insert(a.interpolate_from(b));
                    }
                }
                c_anti_nodes
            })
            .flatten()
            .collect::<HashSet<_>>();
        Ok(World::new(
            antennas,
            anti_nodes,
            length as isize,
            width as isize,
        ))
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let world = World::from_str(input).unwrap();
    let answer = world.anti_node_count();
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut world = World::from_str(input).unwrap();
    world.interpolate_all_anti_nodes();
    let answer = world.anti_node_count();
    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
