use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("day07/src/input.txt").unwrap();
    let mut instructions: Vec<&str> = input.split('\n').collect();
    let mut signals_1 = std::collections::HashMap::new();
    let mut count_1 = 0;
    loop {
        for instruction in &instructions {
            execute(&mut signals_1, instruction);
        }
        count_1 += 1;
        // println!("Result so far: {:?}", signals);
        if let Some(&x) = signals_1.get("a") {
            println!("Puzzle 1: found a = {} in {} iterations.", x, count_1);
            break;
        }
    }

    instructions.insert(0, "3176 -> b");
    let mut signals_2 = std::collections::HashMap::new();
    let mut count_2 = 0;
    loop {
        for instruction in &instructions {
            execute(&mut signals_2, instruction);
        }
        count_2 += 1;
        // println!("Result so far: {:?}", signals);
        if let Some(&x) = signals_2.get("a") {
            println!("Puzzle 2: found a = {} in {} iterations.", x, count_2);
            break;
        }
    }
}

// fn n_mod_m <T: std::ops::Rem<Output = T> + std::ops::Add<Output = T> + Copy>
// (n: T, m: T) -> T {
//     ((n % m) + m) % m
// }

fn extract(signals: &mut HashMap<&str, u16>, wire: &str) -> Option<u16> {
    if let Ok(x) = wire.parse::<u16>() {
        return Some(x);
    } else if let Some(&x) = signals.get(wire) {
        return Some(x);
    } else {
        return None;
    }
}

fn execute<'a, 'b>(
    signals: &'a mut HashMap<&'b str, u16>,
    instruction: &'b str,
) -> &'a mut HashMap<&'b str, u16> {
    let words: Vec<&str> = instruction.split(' ').collect();
    if let Some(_) = signals.get(words.last().unwrap()) {
        return signals; // don't do anything
    }
    if words[1] == "->" {
        if let Some(x) = extract(signals, words[0]) {
            signals.insert(words[2], x);
        }
    } else if words[0] == "NOT" {
        if let Some(x) = extract(signals, words[1]) {
            signals.insert(words[3], !x);
        }
    } else if words[1] == "AND" {
        if let (Some(x), Some(y)) = (extract(signals, words[0]), extract(signals, words[2])) {
            signals.insert(words[4], x & y);
        }
    } else if words[1] == "OR" {
        if let (Some(x), Some(y)) = (extract(signals, words[0]), extract(signals, words[2])) {
            signals.insert(words[4], x | y);
        }
    } else if words[1] == "LSHIFT" {
        if let (Some(x), Some(y)) = (extract(signals, words[0]), extract(signals, words[2])) {
            signals.insert(words[4], x << y);
        }
    } else if words[1] == "RSHIFT" {
        if let (Some(x), Some(y)) = (extract(signals, words[0]), extract(signals, words[2])) {
            signals.insert(words[4], x >> y);
        }
    }
    return signals;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_modulus_1() {
        assert_eq!(n_mod_m(-1, 7), 6);
    }
    #[test]
    fn test_storage() {
        let mut signals = HashMap::new();
        execute(&mut signals, "101 -> za");
        assert_eq!(signals["za"], 101);
    }
    #[test]
    fn test_extract() {
        let mut signals = HashMap::new();
        execute(&mut signals, "5 -> b");
        assert_eq!(extract(&mut signals, "b"), Some(5));
        assert_eq!(extract(&mut signals, "c"), None);
        assert_eq!(extract(&mut signals, "100"), Some(100));
    }
    #[test]
    fn test_not_gate() {
        let mut signals = HashMap::new();
        execute(&mut signals, "NOT 123 -> h");
        assert_eq!(signals["h"], 65412);
        execute(&mut signals, "NOT h -> ij");
        assert_eq!(signals["ij"], 123);
        execute(&mut signals, "NOT v -> w");
        assert_eq!(signals.get("w"), None);
    }
    #[test]
    fn test_and_or_gate() {
        let mut signals = HashMap::new();
        execute(&mut signals, "123 -> x");
        execute(&mut signals, "456 -> y");
        assert_eq!(signals["x"], 123);
        assert_eq!(signals["y"], 456);
        execute(&mut signals, "x AND y -> d");
        execute(&mut signals, "x OR y -> e");
        assert_eq!(signals["d"], 72);
        assert_eq!(signals["e"], 507);
    }
    #[test]
    fn test_shift_gate() {
        let mut signals = HashMap::new();
        execute(&mut signals, "123 -> x");
        execute(&mut signals, "456 -> y");
        assert_eq!(signals["x"], 123);
        assert_eq!(signals["y"], 456);
        execute(&mut signals, "x LSHIFT 2 -> f");
        execute(&mut signals, "y RSHIFT 2 -> g");
        assert_eq!(signals["f"], 492);
        assert_eq!(signals["g"], 114);
    }
}
