fn main() {
    let mut input = String::from("1113122113");
    println!("Iteration 0: {}", &input);
    const N: i32 = 50;
    for i in 1..(N + 1) {
        input = look_and_say(input);
        println!("Iteration {} has length {}", i, input.len());
    }
}

fn look_and_say(input: String) -> String {
    let mut output = "".to_string();
    let mut char_iter = input.chars();
    let mut q = char_iter.next();
    let mut p;
    'outer: loop {
        let mut count = 0;
        if let Some(i) = q {
            p = i;
        } else {
            return output;
        }
        while let Some(i) = q {
            if i != p {
                output += &(count.to_string() + &p.to_string());
                continue 'outer;
            }
            count += 1;
            q = char_iter.next();
        }
        // last contiguous digit span.
        output += &(count.to_string() + &p.to_string());
        return output;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            String::from("311311222113"),
            look_and_say(String::from("1113122113"))
        );
    }
}
