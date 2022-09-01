fn main() {
    let secret_key = "a";
    puzzle_1(secret_key);
    puzzle_2(secret_key);
}

fn puzzle_1(secret_key: &str) {
    let mut answer = 0;
    loop {
        if first_n_of_md5(&(secret_key.to_owned() + &answer.to_string()), 5) == "00000" {
            println!("Answer to Puzzle 1: {}", answer);
            break;
        }
        answer += 1;
        if answer % 10000 == 0 {
            println!("Searching: {}", answer);
        }
    }
}

fn puzzle_2(secret_key: &str) {
    let mut answer = 0;
    loop {
        if first_n_of_md5(&(secret_key.to_owned() + &answer.to_string()), 6) == "000000" {
            println!("Answer to Puzzle 2: {}", answer);
            break;
        }
        answer += 1;
        if answer % 10000 == 0 {
            println!("Searching: {}", answer);
        }
    }
}

fn first_n_of_md5(input: &str, n: usize) -> String {
    let digest = md5::compute(input);
    let output = format!("{:X}", digest);
    return output[..n].to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!("00000", first_5_of_md5("abcdef609043"));
        assert_eq!("00000", first_5_of_md5("pqrstuv1048970"));
    }
}
