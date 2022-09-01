use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("day09/src/input.txt").unwrap();

    let mut distances = HashMap::new();
    let mut cities = HashSet::new();
    for line in input.lines() {
        let words: Vec<&str> = line.split(' ').collect();
        let city1 = words[0];
        let city2 = words[2];
        cities.insert(city1);
        cities.insert(city2);
        let distance = words[4].parse::<usize>().unwrap();
        distances.insert((city1, city2), distance);
        distances.insert((city2, city1), distance);
    }
    let mut min_distance = usize::MAX;
    let mut max_distance = 0;
    let mut min_perm: Vec<&&str> = vec![];
    let mut max_perm: Vec<&&str> = vec![];
    for perm in cities.iter().permutations(cities.len()).unique() {
        let mut curr_distance = 0;
        for i in 0..cities.len() - 1 {
            curr_distance += distances[&(*perm[i], *perm[i + 1])];
        }
        println!("Route {:?} has distance {}.", perm, curr_distance);
        if curr_distance < min_distance {
            min_distance = curr_distance;
            min_perm = perm;
        } else if curr_distance > max_distance {
            max_distance = curr_distance;
            max_perm = perm;
        }
    }
    println!(
        "Minimum route is {:?} with distance {}.",
        min_perm, min_distance
    );
    println!(
        "Maximum route is {:?} with distance {}.",
        max_perm, max_distance
    );
}
