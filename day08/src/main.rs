use std::fs;

fn main() {
    let input = fs::read_to_string("day08/src/input.txt").unwrap();
    let lines = input.lines();
    let mut count_1 = 0;
    let mut count_2 = 0;
    for line in lines {
        count_1 += puzzle_1(line);
        count_2 += puzzle_2(line);
    }
    println!("The answer to puzzle 1 is {}", count_1);
    println!("The answer to puzzle 2 is {}", count_2);
}

fn puzzle_1(line: &str) -> usize {
    let mut char_iter = line.chars();
    let mut p = char_iter.next();
    let mut q = char_iter.next();
    let mut count = 0;
    while let (Some(i), Some(j)) = (p, q) {
        if i == '\\' {
            if j == '\\' || j == '"' {
                count += 1;
                p = char_iter.next();
                q = char_iter.next();
                continue;
            }
            if j == 'x' {
                count += 3;
                char_iter.next();
                char_iter.next();
                p = char_iter.next();
                q = char_iter.next();
                continue;
            }
        }
        p = q;
        q = char_iter.next();
    }
    return count + 2;
}

fn puzzle_2(line: &str) -> usize {
    return line.matches(r#"""#).count() + line.matches(r#"\"#).count() + 2;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(2, puzzle_1(r#""""#));
        assert_eq!(2, puzzle_1(r#""abc""#));
        assert_eq!(3, puzzle_1(r#""aaa\"aaa""#));
        assert_eq!(5, puzzle_1(r#""\x27""#));
        assert_eq!(4, puzzle_1(r#""\\\\""#));
    }
}
