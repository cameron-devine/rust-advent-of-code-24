use std::collections::HashSet;
use std::str::FromStr;

advent_of_code::solution!(9);

#[derive(Debug)]
struct DiskMapParseError;
#[derive(Debug)]
struct DiskMapDeFragmentError;
type FileId = u64;
struct DiskMap {
    map: Vec<Option<FileId>>
}

impl FromStr for DiskMap {
    type Err = DiskMapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: Vec<Option<FileId>> = Vec::new();
        let mut id_count = 0;
        let mut iter = s.chars();
        while let Some(files) = iter.next() {
            let free_space = iter.next();
            let file_count = files.to_digit(10).expect("File count is invalid");
            for _ in 0..file_count {
                map.push(Some(id_count));
            }

            match free_space {
                Some(free_space) => {
                    let free_space_count = free_space.to_digit(10).expect("Free space is invalid");
                    for _ in 0..free_space_count {
                        map.push(None);
                    }
                }
                _ => {}
            }
            id_count += 1;
        }

        Ok(DiskMap::new( map ))
    }
}
impl DiskMap {

    fn new(map: Vec<Option<FileId>>) -> DiskMap {
        DiskMap { map }
    }
    fn de_fragment_single_blocks(&mut self) -> Result<(), DiskMapDeFragmentError> {
        let mut front = 0;
        let mut back = self.map.len() - 1;

        while front <= back {
            while self.map[front].is_some() {
                front += 1;
            }

            while self.map[back].is_none() {
                back -= 1;
            }
            if front < back {
                self.map.swap(front, back);
            }
        }
        Ok(())
    }

    fn de_fragment_whole_files(&mut self) -> Result<(), DiskMapDeFragmentError> {

        let range_of_free_space_with_size = |copy: &Vec<Option<FileId>>, size: usize, upper_limit: usize| -> Option<(usize, usize)> {
            let mut free_space_front = copy.iter().position(|x| x.is_none()).unwrap();
            let mut free_space_back = free_space_front;

            while free_space_front < upper_limit {
                while copy[free_space_back + 1].is_none() {
                    free_space_back += 1;
                }
                if (free_space_back - free_space_front + 1) >= size {
                    return Some((free_space_front, free_space_back));
                }

                free_space_front = free_space_back + 1;
                while copy[free_space_front].is_some() {
                    free_space_front += 1;
                }
                free_space_back = free_space_front;
            }

            None
        };

        let mut moved_ids: HashSet<FileId> = HashSet::new();
        let mut file_end = self.map.len() - 1;
        let mut file_start = file_end;
        while file_end > 0  && file_start > 0 {
            while self.map[file_end].is_none() {
                file_end -= 1;
                file_start = file_end;
            }

            while file_start > 0 && self.map[file_start - 1] == self.map[file_end] {
                file_start -= 1;
            }

            if moved_ids.contains(&self.map[file_start].unwrap()) {
                // println!("File already moved");
            } else {
                let free_window = range_of_free_space_with_size(&self.map, file_end - file_start + 1, file_start);
                if free_window.is_some() {
                    let mut free_window_start = free_window.unwrap().0;
                    for i in file_start..file_end + 1 {
                        self.map.swap(i, free_window_start);
                        free_window_start += 1;
                    }
                    moved_ids.insert(self.map[free_window_start-1].unwrap());
                }
                }

            if file_start > 0 {
                file_end = file_start - 1;
                file_start = file_end;
            }

        }
        Ok(())
    }
    fn checksum(&self) -> u64 {
        let mut checksum = vec![];
        for (i, file) in self.map.iter().enumerate() {
            match file {
                Some(id) => checksum.push(i as FileId * id),
                None => ()
            };
        }
        checksum.iter().sum()
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let mut disk_map = DiskMap::from_str(input).ok()?;
    disk_map.de_fragment_single_blocks().expect("Error de fragmenting.");
    Some(disk_map.checksum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk_map = DiskMap::from_str(input).ok()?;
    disk_map.de_fragment_whole_files().expect("Error de fragmenting.");
    Some(disk_map.checksum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
