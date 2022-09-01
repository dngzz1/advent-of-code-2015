use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("day13/src/input.txt").unwrap();
    let mut map = HashMap::new();
    let mut people = HashSet::new();
    for line in input.lines() {
        let words: Vec<&str> = line.split(' ').collect();
        let person1 = words[0];
        let person2 = &words[10][0..words[10].len() - 1];
        let magnitude = words[3].parse::<isize>().unwrap();
        let mut sign = 0;
        if words[2] == "gain" {
            sign = 1;
        } else if words[2] == "lose" {
            sign = -1;
        }
        let happiness = sign * magnitude;
        map.insert((person1, person2), happiness);
        people.insert(person1);
    }
    println!("Puzzle 1:");
    solve(&map, &people);
    for &person in &people {
        map.insert(("me", person), 0);
        map.insert((person, "me"), 0);
    }
    people.insert("me");
    println!("Puzzle 2:");
    solve(&map, &people);
}

fn solve(map: &HashMap<(&str, &str), isize>, people: &HashSet<&str>) {
    let mut max_happiness = isize::MIN;
    let mut max_perm = vec![];
    for perm in people.iter().permutations(people.len()).unique() {
        let mut total_happiness = 0;
        for i in 0..people.len() - 1 {
            total_happiness += map.get(&(*perm[i], *perm[i + 1])).unwrap();
            total_happiness += map.get(&(*perm[i + 1], *perm[i])).unwrap();
        }
        total_happiness += map.get(&(*perm[0], *perm[perm.len() - 1])).unwrap();
        total_happiness += map.get(&(*perm[perm.len() - 1], *perm[0])).unwrap();
        if total_happiness > max_happiness {
            max_happiness = total_happiness;
            max_perm = perm;
        }
    }
    println!("{:?} gives total happiness of {}.", max_perm, max_happiness);
}
