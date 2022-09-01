use crate::light::Light;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Grid {
    size: isize,
    map: HashMap<(isize, isize), Light>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut s = String::new();
        for i in 0..self.size {
            for j in 0..self.size {
                s.push(self.map.get(&(i, j)).unwrap().symbol());
            }
            if i < self.size - 1 {
                s.push('\n');
            }
        }
        write!(f, "{}", s.as_str())
    }
}

impl Grid {
    pub fn new(size: isize, data: &str) -> Self {
        let mut map = HashMap::new();
        let mut i = 0;
        let mut j = 0;
        for line in data.lines() {
            for c in line.chars() {
                match c {
                    '#' => map.insert((i, j), Light::On),
                    '.' => map.insert((i, j), Light::Off),
                    z => panic!("data contains unrecognised symbol {}", z),
                };
                j += 1;
            }
            if j != size {
                panic!(
                    "Incorrect number of symbols on line {}: expecting {}, got {}",
                    i, size, j
                );
            }
            i += 1;
            j = 0;
        }
        if i != size {
            panic!("Incorrect number of lines: expecting {}, got {}", size, i);
        }
        Self { size, map }
    }

    #[cfg(test)]
    pub fn to_string(&self) -> String {
        format!("{}", &self)
    }

    fn count_neighbors(&self, pos: (isize, isize)) -> usize {
        let mut count = 0;
        let cardinal = [
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ];
        for dir in cardinal {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            let state = self.map.get(&new_pos).unwrap_or(&Light::Off);
            match *state {
                Light::On => count += 1,
                Light::Off => {}
            }
        }
        return count;
    }

    pub fn step(&mut self) {
        let mut num_neighbors = HashMap::new();
        for i in 0..self.size {
            for j in 0..self.size {
                num_neighbors.insert((i, j), self.count_neighbors((i, j)));
            }
        }
        for i in 0..self.size {
            for j in 0..self.size {
                let n = *num_neighbors
                    .get(&(i, j))
                    .expect("error counting neighbors");
                match self.map.get(&(i, j)).expect("error getting element") {
                    Light::On => {
                        if n != 2 && n != 3 {
                            self.map.insert((i, j), Light::Off);
                        }
                    }
                    Light::Off => {
                        if n == 3 {
                            self.map.insert((i, j), Light::On);
                        }
                    }
                }
            }
        }
    }

    pub fn step2(&mut self) {
        let n = self.size;
        self.map.insert((0, 0), Light::On);
        self.map.insert((n - 1, 0), Light::On);
        self.map.insert((0, n - 1), Light::On);
        self.map.insert((n - 1, n - 1), Light::On);
        self.step();
        self.map.insert((0, 0), Light::On);
        self.map.insert((n - 1, 0), Light::On);
        self.map.insert((0, n - 1), Light::On);
        self.map.insert((n - 1, n - 1), Light::On);
    }

    pub fn count_total_alive(&self) -> isize {
        let mut count = 0;
        for (_, value) in &self.map {
            match value {
                &Light::On => {
                    count += 1;
                }
                &Light::Off => {}
            }
        }
        return count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_count_neighbors() {
        let size = 3;
        let data = "...\n.#.\n###";
        let grid = Grid::new(size, data);
        assert_eq!(grid.count_neighbors((0, 0)), 1);
        assert_eq!(grid.count_neighbors((0, 1)), 1);
        assert_eq!(grid.count_neighbors((0, 2)), 1);
        assert_eq!(grid.count_neighbors((1, 0)), 3);
        assert_eq!(grid.count_neighbors((1, 1)), 3);
    }

    #[test]
    fn can_step() {
        let size = 6;
        let data = r#".#.#.#
...##.
#....#
..#...
#.#..#
####.."#;
        let mut grid = Grid::new(size, data);
        grid.step();
        let expected = r#"..##..
..##.#
...##.
......
#.....
#.##.."#;
        assert_eq!(expected, grid.to_string());
    }

    #[test]
    fn can_count_total_alive() {
        let size = 6;
        let data = r#".#.#.#
...##.
#....#
..#...
#.#..#
####.."#;
        let mut grid = Grid::new(size, data);
        assert_eq!(15, grid.count_total_alive());
        grid.step();
        assert_eq!(11, grid.count_total_alive());
    }
}
