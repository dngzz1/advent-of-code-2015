use std::fs;

fn main() {
    let input = fs::read_to_string("day17/src/input.txt").unwrap();
    let mut jugs = vec![];
    for line in input.lines() {
        jugs.push(line.parse::<i32>().unwrap());
    }
    let target = 150;
    println!("Puzzle 1: {}", puzzle_1(target, &jugs));
    println!("Puzzle 2: {}", puzzle_2(target, &jugs, 4));
}

fn puzzle_1(target: i32, jugs: &Vec<i32>) -> i32 {
    if target < 0 {
        return 0;
    }
    if target == 0 {
        return 1;
    }
    if jugs.len() == 1 && jugs[0] == target {
        return 1;
    }
    if jugs.len() == 1 && jugs[0] != target {
        return 0;
    }
    let mut jugs_truncated = jugs.clone();
    jugs_truncated.remove(0);
    return puzzle_1(target - jugs[0], &jugs_truncated) + puzzle_1(target, &jugs_truncated);
}

fn puzzle_2(target: i32, jugs: &Vec<i32>, cap: i32) -> i32 {
    if target < 0 {
        return 0;
    }
    if target == 0 && cap >= 0 {
        return 1;
    }
    if jugs.len() == 1 && jugs[0] != target {
        return 0;
    }
    if cap <= 0 {
        return 0;
    }
    if jugs.len() == 1 && jugs[0] == target {
        return 1;
    }
    let mut jugs_truncated = jugs.clone();
    jugs_truncated.remove(0);
    return puzzle_2(target - jugs[0], &jugs_truncated, cap - 1)
        + puzzle_2(target, &jugs_truncated, cap);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_jug() {
        let target = 4;
        let jugs = vec![4];
        assert_eq!(1, puzzle_1(target, &jugs));
    }
    #[test]
    fn one_wrong_jug() {
        let target = 10;
        let jugs = vec![5];
        assert_eq!(0, puzzle_1(target, &jugs));
    }

    #[test]
    fn two_jugs() {
        let target = 10;
        let jugs = vec![4, 6];
        assert_eq!(1, puzzle_1(target, &jugs));
    }

    #[test]
    fn three_jugs() {
        let target = 10;
        let jugs = vec![10, 5, 5];
        assert_eq!(2, puzzle_1(target, &jugs));
    }

    #[test]
    fn three_wrong_jugs() {
        let target = 10;
        let jugs = vec![9, 6, 5];
        assert_eq!(0, puzzle_1(target, &jugs));
    }
}
