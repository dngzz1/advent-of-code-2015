use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("day03/src/input.txt").unwrap();
    println!(
        "Number of houses receiving at least one present by Santa alone: {}",
        puzzle_1(&input)
    );
    println!(
        "Number of houses receiving at least one present by Santa + Robo: {}",
        puzzle_2(&input)
    );
}

fn puzzle_1(input: &str) -> usize {
    let mut num_houses = 1;
    let mut curr_pos = (0, 0);
    let mut presents: HashMap<(i32, i32), usize> = HashMap::new();
    presents.insert(curr_pos, 1);
    for c in input.chars() {
        match c {
            '^' => curr_pos.1 += 1,
            '>' => curr_pos.0 += 1,
            'v' => curr_pos.1 -= 1,
            '<' => curr_pos.0 -= 1,
            _ => {}
        }
        if let Some(n) = presents.get_mut(&curr_pos) {
            *n += 1;
        } else {
            presents.insert(curr_pos, 1);
            num_houses += 1;
        }
    }
    return num_houses;
}

fn puzzle_2(input: &str) -> usize {
    let mut num_houses = 1;
    let mut curr_pos_santa = (0, 0);
    let mut curr_pos_robo = (0, 0);
    let mut robos_turn = false;
    let mut presents: HashMap<(i32, i32), usize> = HashMap::new();
    presents.insert((0, 0), 1);
    for c in input.chars() {
        if robos_turn {
            match c {
                '^' => curr_pos_robo.1 += 1,
                '>' => curr_pos_robo.0 += 1,
                'v' => curr_pos_robo.1 -= 1,
                '<' => curr_pos_robo.0 -= 1,
                _ => {}
            }
            if let Some(n) = presents.get_mut(&curr_pos_robo) {
                *n += 1;
            } else {
                presents.insert(curr_pos_robo, 1);
                num_houses += 1;
            }
        } else {
            match c {
                '^' => curr_pos_santa.1 += 1,
                '>' => curr_pos_santa.0 += 1,
                'v' => curr_pos_santa.1 -= 1,
                '<' => curr_pos_santa.0 -= 1,
                _ => {}
            }
            if let Some(n) = presents.get_mut(&curr_pos_santa) {
                *n += 1;
            } else {
                presents.insert(curr_pos_santa, 1);
                num_houses += 1;
            }
        }
        robos_turn = !robos_turn;
    }
    return num_houses;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn go_right() {
        assert_eq!(2, puzzle_1(">"));
    }
    #[test]
    fn loop_round() {
        assert_eq!(4, puzzle_1("^>v<"));
    }
    #[test]
    fn hesitant() {
        assert_eq!(2, puzzle_1("^v^v^v^v"));
    }
    #[test]
    fn north_south_divide() {
        assert_eq!(3, puzzle_2("^v"));
    }
    #[test]
    fn go_back() {
        assert_eq!(3, puzzle_2("^>v<"));
    }
    #[test]
    fn diverge() {
        assert_eq!(11, puzzle_2("^v^v^v^v^v"));
    }
}
