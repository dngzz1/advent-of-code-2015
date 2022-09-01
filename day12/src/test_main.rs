use crate::*;

#[test]
fn no_reds_once() {
    let mut input = r#"{a:3,b:4}"#.to_string();
    delete_red_once(&mut input).unwrap();
    assert_eq!(r#"{a:3,b:4}"#, input);
}

#[test]
fn simple_red_once() {
    let mut input = r#"{a:10, {"a":"red"}}"#.to_string();
    delete_red_once(&mut input).unwrap();
    assert_eq!(r#"{a:10, }"#, input);
}

#[test]
fn medium_red_once() {
    let mut input = r#"{a:10, {"a":"red", {}, b:4}}"#.to_string();
    delete_red_once(&mut input).unwrap();
    assert_eq!(r#"{a:10, }"#, input);
}

#[test]
fn harder_red_once() {
    let mut input = r#"{a:10, {}, {{b:100},"a":"red", {}, b:4}}"#.to_string();
    delete_red_once(&mut input).unwrap();
    assert_eq!(r#"{a:10, {}, }"#, input);
}

#[test]
fn can_advance_by() {
    let a = [1, 2, 3, 4];
    let mut iter = a.iter();

    assert_eq!(iter.advance_by(2), Ok(()));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.advance_by(0), Ok(()));
    assert_eq!(iter.advance_by(100), Err(1)); // only `&4` was skipped
}

#[test]
fn no_reds() {
    let mut input = r#"{a:3,b:4}"#.to_string();
    delete_red(&mut input).unwrap();
    assert_eq!(r#"{a:3,b:4}"#, input);
}

#[test]
fn simple_red() {
    let mut input = r#"{a:10, {"a":"red"}}"#.to_string();
    delete_red(&mut input).unwrap();
    assert_eq!(r#"{a:10, }"#, input);
}

#[test]
fn medium_red() {
    let mut input = r#"{a:10, {"a":"red", {}, b:4}}"#.to_string();
    delete_red(&mut input).unwrap();
    assert_eq!(r#"{a:10, }"#, input);
}

#[test]
fn harder_red() {
    let mut input = r#"{a:10, {}, {{b:100},"a":"red", {}, b:4}}"#.to_string();
    delete_red(&mut input).unwrap();
    assert_eq!(r#"{a:10, {}, }"#, input);
}

#[test]
fn two_reds() {
    let mut input = r#"{a:10, {}, {{b:100},"a":"red", {}, b:4}, {"a":"red"}}"#.to_string();
    delete_red(&mut input).unwrap();
    assert_eq!(r#"{a:10, {}, , }"#, input);
}

#[test]
fn sum_two_reds() {
    let mut input = r#"{a:10, b:3, {}, {{b:100},"a":"red", {}, b:4}, {"a":"red"}}"#.to_string();
    delete_red(&mut input).unwrap();
    assert_eq!(13, sum_digits(&input));
}

#[test]
fn test_1() {
    let mut input = r#"[1,{"c":"red","b":2},3]"#.to_string();
    delete_red(&mut input).unwrap();
    assert_eq!(4, sum_digits(&input));
}

#[test]
fn test_2() {
    let mut input = r#"[1,"red",5]"#.to_string();
    delete_red(&mut input).unwrap();
    assert_eq!(6, sum_digits(&input));
}
