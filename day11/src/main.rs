use std::collections::HashMap;

fn main() {
    let mut password = String::from("hepxxzaa");
    while !(contains_contiguous_3(&password)
        && !contains_iol(&password)
        && contains_nonoverlapping_pair(&password))
    {
        password = increment(password);
        println!("{}", &password);
    }
}

fn increment_char(c: char) -> char {
    let mut legal_letters = "abcdefghjkmnpqrstuvwxyza".chars();
    let mut map = HashMap::new();
    let mut p = legal_letters.next().unwrap();
    while let Some(i) = legal_letters.next() {
        map.insert(p, i);
        p = i;
    }
    return map[&c];
}

fn increment(password: String) -> String {
    let mut password_chars: Vec<char> = password.chars().collect();
    let mut j = password_chars.len() - 1;
    loop {
        password_chars[j] = increment_char(password_chars[j]);
        if j == 0 {
            break;
        } else if password_chars[j] == 'a' {
            j -= 1;
        } else {
            break;
        }
    }
    return password_chars.iter().collect();
}

fn contains_contiguous_3(password: &str) -> bool {
    const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
    let mut threes = vec![];
    for i in 0..(26 - 2) {
        threes.push(&ALPHABET[i..(i + 3)]);
    }
    for i in 0..(password.len() - 2) {
        if threes.contains(&&password[i..(i + 3)]) {
            return true;
        }
    }
    return false;
}

fn contains_iol(password: &str) -> bool {
    for c in password.chars() {
        if c == 'i' || c == 'o' || c == 'l' {
            return true;
        }
    }
    return false;
}

fn contains_nonoverlapping_pair(password: &str) -> bool {
    if password.len() < 4 {
        return false;
    }
    let mut char_iter = password.chars();
    let mut p = char_iter.next();
    let mut q = char_iter.next();
    let mut count = 0;
    while let (Some(i), Some(j)) = (p, q) {
        if i == j {
            count += 1;
            p = char_iter.next();
            q = char_iter.next();
        } else {
            p = q;
            q = char_iter.next();
        }
    }
    return count >= 2;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_contiguous_3() {
        assert_eq!(true, contains_contiguous_3("abc"));
        assert_eq!(true, contains_contiguous_3("ajkla"));
        assert_eq!(false, contains_contiguous_3("zabd"));
    }
    #[test]
    fn test_nonoverlapping_pair() {
        assert_eq!(true, contains_nonoverlapping_pair("aabb"));
    }
    #[test]
    fn test_nonoverlapping_pair_2() {
        assert_eq!(false, contains_nonoverlapping_pair("aaa"));
    }
    #[test]
    fn test_increment_char() {
        assert_eq!('d', increment_char('c'));
        assert_eq!('a', increment_char('z'));
    }
    #[test]
    fn test_increment() {
        assert_eq!("hxzzb", increment(String::from("hxzza")));
        assert_eq!("hxzba", increment(String::from("hxzaz")));
        assert_eq!("hxzaa", increment(String::from("hxyzz")));
        assert_eq!("abj", increment(String::from("abh")));
        assert_eq!("aaa", increment(String::from("zzz")));
    }
}
