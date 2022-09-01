use std::fs;

fn main() {
    let input = fs::read_to_string("day05/src/input.txt").unwrap();
    let words = input.lines();
    let mut count_1 = 0;
    let mut count_2 = 0;
    for word in words {
        if is_nice_1(word) {
            count_1 += 1;
        }
        if is_nice_2(word) {
            count_2 += 1;
        }
    }
    println!("Puzzle 1: There are {} nice words.", count_1);
    println!("Puzzle 2: There are {} nice words.", count_2);
}

fn is_nice_1(word: &str) -> bool {
    if word.len() < 3 {
        return false;
    }
    let mut char_iter = word.chars();
    let mut p = char_iter.next();
    let mut q = char_iter.next();
    let mut vowel_count = 0;
    let mut double_count = 0;
    if vec!['a', 'e', 'i', 'o', 'u'].contains(&p.unwrap()) {
        vowel_count += 1;
    }
    while let (Some(i), Some(j)) = (p, q) {
        if vec![('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')].contains(&(i, j)) {
            return false;
        }
        if vec!['a', 'e', 'i', 'o', 'u'].contains(&j) {
            vowel_count += 1;
        }
        if i == j {
            double_count += 1;
        }
        // move forward.
        p = q;
        q = char_iter.next();
    }
    if double_count >= 1 && vowel_count >= 3 {
        return true;
    }
    return false;
}

fn is_nice_2(word: &str) -> bool {
    if word.len() < 4 {
        return false;
    }
    let mut char_iter = word.chars();
    let mut p = char_iter.next();
    let mut q = char_iter.next();
    let mut r = char_iter.next();
    let mut sandwich_count = 0;
    let mut pairs = vec![(p.unwrap(), q.unwrap())];
    let mut found_pair = false;
    while let (Some(i), Some(j), Some(k)) = (p, q, r) {
        if p == r {
            sandwich_count += 1;
        }
        if pairs.contains(&(j, k)) && !(i == j && j == k) {
            found_pair = true;
        } else {
            pairs.push((j, k));
        }
        // move forward
        p = q;
        q = r;
        r = char_iter.next()
    }
    if sandwich_count >= 1 && found_pair {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert!(is_nice_1("ugknbfddgicrmopn"));
    }
    #[test]
    fn test_2() {
        assert!(is_nice_1("aaa"));
    }
    #[test]
    fn test_3() {
        assert!(!is_nice_1("jchzalrnumimnmhp"));
    }
    #[test]
    fn test_4() {
        assert!(!is_nice_1("haegwjzuvuyypxyu"));
    }
    #[test]
    fn test_5() {
        assert!(!is_nice_1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_6() {
        assert!(is_nice_2("qjhvhtzxzqqjkmpb"));
    }

    #[test]
    fn test_7() {
        assert!(is_nice_2("xxyxx"));
    }

    #[test]
    fn test_8() {
        assert!(!is_nice_2("uurcxstgmygtbstg"));
    }

    #[test]
    fn test_9() {
        assert!(!is_nice_2("ieodomkazucvgmuy"));
    }

    #[test]
    fn test_10() {
        assert!(!is_nice_2("ieodomkazucvgmuyooo"));
    }
}
