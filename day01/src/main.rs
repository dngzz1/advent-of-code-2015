use std::fs;

pub fn main() {
    println!("Solving day 01 of 2015...");
    let instruction = fs::read_to_string("day01/src/input.txt").unwrap();
    println!("Current floor: {}", calculate_floor(&instruction));
    println!(
        "First time to hit basement: {}",
        first_time_to_hit_basement(&instruction).unwrap()
    );
}

fn calculate_floor(instruction: &str) -> isize {
    let mut answer = 0;
    for c in instruction.chars() {
        if c == '(' {
            answer += 1;
        } else if c == ')' {
            answer -= 1;
        }
    }
    answer
}

fn first_time_to_hit_basement(instruction: &str) -> Option<usize> {
    let mut time = 0;
    let mut level = 0;
    for c in instruction.chars() {
        if c == '(' {
            level += 1;
        } else if c == ')' {
            level -= 1;
        }
        time += 1;
        if level < 0 {
            return Some(time);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_floor() {
        assert_eq!(1, calculate_floor("(()"));
    }

    #[test]
    fn test_first_time_to_hit_basement() {
        assert_eq!(Some(1), first_time_to_hit_basement(")(("));
        assert_eq!(None, first_time_to_hit_basement("((()"));
        assert_eq!(Some(5), first_time_to_hit_basement("()())"));
    }
}
