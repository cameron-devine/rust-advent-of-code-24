use crate::Direction::{Down, Left, Right, Up};
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(6);

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
struct Coordinates(isize, isize);

impl Coordinates {
    fn off_grid(&self, length: usize, width: usize) -> bool {
        self.0 < 0 || self.0 >= length as isize || self.1 < 0 || self.1 >= width as isize
    }
}
#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Clone, Copy)]
struct Guard {
    loc: Coordinates,
    direction: Direction,
}

impl Guard {
    fn turn_right(&mut self) -> () {
        use Direction::*;
        self.direction = match self.direction {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }

    fn move_forward(&mut self) -> () {
        self.loc = self.next_space()
    }

    fn next_space(&self) -> Coordinates {
        match self.direction {
            Up => Coordinates(self.loc.0 - 1, self.loc.1),
            Down => Coordinates(self.loc.0 + 1, self.loc.1),
            Left => Coordinates(self.loc.0, self.loc.1 - 1),
            Right => Coordinates(self.loc.0, self.loc.1 + 1),
        }
    }

    fn turn_or_move(&mut self, obstacles: &Vec<Coordinates>) -> () {
        if obstacles.contains(&self.next_space()) {
            self.turn_right()
        } else {
            self.move_forward()
        }
    }
}

#[derive(Debug, Clone)]
struct CycleError;

#[derive(Clone)]
struct World {
    obstacles: Vec<Coordinates>,
    guard: Option<Guard>,
    visited_locations: HashMap<Coordinates, HashSet<Direction>>,
    width: usize,
    length: usize,
}

impl World {
    fn new(str: &str) -> World {
        let mut obstacles: Vec<Coordinates> = Vec::new();
        let mut guard: Option<Guard> = None;
        let mut visited_locations: HashMap<Coordinates, HashSet<Direction>> = HashMap::new();
        let mut length: usize = 0;
        let mut width: usize = 0;
        for (i, row) in str.lines().enumerate() {
            for (j, c) in row.chars().enumerate() {
                use Direction::*;
                match c {
                    '#' => {
                        obstacles.push(Coordinates(i as isize, j as isize));
                    }
                    '^' => {
                        guard = Some(Guard {
                            loc: Coordinates(i as isize, j as isize),
                            direction: Up,
                        });
                        visited_locations.entry(Coordinates(i as isize, j as isize)).or_insert(HashSet::new()).insert(Up);
                    }
                    '>' => {
                        guard = Some(Guard {
                            loc: Coordinates(i as isize, j as isize),
                            direction: Right,
                        });
                        visited_locations.entry(Coordinates(i as isize, j as isize)).or_insert(HashSet::new()).insert(Right);
                    }
                    'v' => {
                        guard = Some(Guard {
                            loc: Coordinates(i as isize, j as isize),
                            direction: Down,
                        });
                        visited_locations.entry(Coordinates(i as isize, j as isize)).or_insert(HashSet::new()).insert(Down);
                    }
                    '<' => {
                        guard = Some(Guard {
                            loc: Coordinates(i as isize, j as isize),
                            direction: Left,
                        });
                        visited_locations.entry(Coordinates(i as isize, j as isize)).or_insert(HashSet::new()).insert(Left);
                    }
                    _ => (),
                }
                width = j+1;
            }
            length = i+1;
        }

        World {
            obstacles,
            guard,
            visited_locations,
            width,
            length,
        }
    }

    fn run(&mut self) -> Result<(), CycleError> {
        while self.guard.is_some() {
            let mut guard = self.guard.take().unwrap();
            guard.turn_or_move(&self.obstacles);
            let loc = guard.loc;
            let direction = guard.direction;
            if !loc.off_grid(self.length, self.width) {
                let entry = self.visited_locations.entry(loc).or_insert(HashSet::new());
                if entry.contains(&direction) {
                    return Err(CycleError);
                } else {
                    entry.insert(direction);
                }
            }
            if loc.off_grid(self.length, self.width)
            {
                self.guard = None;
            } else {
                self.guard = Some(guard)
            }
        }
        Ok(())
    }

    fn find_cyclical_obstacles(&mut self) -> Vec<Coordinates> {
        let mut cycle_coords = vec![];
        let mut copy = self.clone();
        copy.run().expect("Unable to run the world");
        let visited_locations = &copy.visited_locations;
        println!("Checking {} valid path locations.", visited_locations.len());
        let mut index: f64 = 0.0;
        for coord in visited_locations.keys() {
            let i = coord.0 as isize;
            let j = coord.1 as isize;
            let coords = Coordinates(i as isize, j as isize);

            // We don't want to go adding a bunch of obstacles to the original world
            let mut world_clone = self.clone();
            world_clone.obstacles.push(coords);
            let result = world_clone.run();
            if result.is_err() {
                cycle_coords.push(coords);
            }
            index += 1.0;
            println!("%{}", index / visited_locations.len() as f64 * 100.0);
        }

        cycle_coords
    }

    fn visited_count(&self) -> u32 {
        self.visited_locations.len() as u32
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut world = World::new(input);
    world.run().expect("PANIC CYCLE DETECTED");
    Some(world.visited_count())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut world = World::new(input);
    Some(world.find_cyclical_obstacles().len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
