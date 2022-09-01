#![feature(iter_advance_by)]

use regex::Regex;
use std::fs;
// use anyhow::Result;
use std::io::{Result, Error, ErrorKind};

macro_rules! unwrap_or_return {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return Err(Error::new(ErrorKind::Other, "cannot advance")),
        }
    }
}


fn main() {
    let mut input = fs::read_to_string("day12/src/input.txt").unwrap();
    println!("Puzzle 1: Sum = {}", sum_digits(&input));
    delete_red(&mut input).expect("Can't find open or close braces for red");
    println!("Puzzle 2: Sum = {}", sum_digits(&input));
}

fn sum_digits(input: &str) -> i32 {
    extract_digits(input).iter().sum::<i32>()
}

fn extract_digits(junk: &str) -> Vec<i32> {
    let pattern = Regex::new(r#"-?[0-9]+"#).unwrap();
    return pattern
        .find_iter(junk)
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .collect();
}

fn delete_red(input: &mut String) -> Result<()> {
    if delete_red_once(input)? {
        delete_red(input)?;
    }
    Ok(())
}

fn delete_red_once(input: &mut String) -> Result<bool> {
    let index_red = match input.find(":\"red\"") {
        None => return Ok(false),
        Some(i) => i,
    };
    let mut iter_backwards = input.chars().rev();
    unwrap_or_return! (iter_backwards.advance_by(input.len() - index_red));
    let index_start = index_red - get_index_of_braces(iter_backwards, false);
    let mut iter_forwards = input.chars();
    unwrap_or_return! (iter_forwards.advance_by(index_red));
    let index_end = index_red + get_index_of_braces(iter_forwards, true);
    let mut output = input.to_string();
    output.replace_range(index_start..index_end, "");
    *input = output;
    Ok(true)
}

fn get_index_of_braces<I: Iterator<Item=char>>(mut iter: I, forwards: bool) -> usize {
    let mut count = 0;
    let mut index = 0;
    while count >= 0 {
        let c = iter.next().unwrap();
        index += 1;
        match (forwards, c) {
            (true, '{') => count += 1,
            (true, '}') => count -= 1,
            (false, '{') => count -= 1,
            (false, '}') => count += 1,
            _ => (),
        }
    }
    index
}

#[cfg(test)]
mod test_main;
