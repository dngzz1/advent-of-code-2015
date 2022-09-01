mod grid;
mod light;

use grid::*;
use std::fs;

fn main() {
    let data = fs::read_to_string("day18/src/input.txt").expect("file not found");
    let mut grid = Grid::new(100, data.as_str());
    for _ in 0..100 {
        grid.step();
    }
    println!("Puzzle 1: Total alive = {}", grid.count_total_alive());

    grid = Grid::new(100, data.as_str());
    for _ in 0..100 {
        grid.step2();
    }
    println!("Puzzle 2: Total alive = {}", grid.count_total_alive());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn two_by_two() {
        let size = 2;
        let data = "..\n.#";
        let grid = Grid::new(size, data);
    }
}
