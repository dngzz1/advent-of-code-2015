use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("day16/src/input.txt").unwrap();
    let mut maps: Vec<HashMap<&str, i32>> = vec![];
    for line in input.lines() {
        let words: Vec<&str> = line.split(' ').collect();
        let thing_1 = &words[2][..words[2].len() - 1];
        let amount_1 = words[3][..words[3].len() - 1].parse::<i32>().unwrap();
        let thing_2 = &words[4][..words[4].len() - 1];
        let amount_2 = words[5][..words[5].len() - 1].parse::<i32>().unwrap();
        let thing_3 = &words[6][..words[6].len() - 1];
        let amount_3 = words[7].parse::<i32>().unwrap();
        let mut map = HashMap::new();
        map.insert(thing_1, amount_1);
        map.insert(thing_2, amount_2);
        map.insert(thing_3, amount_3);
        maps.push(map);
    }
    let mut real_map = HashMap::new();
    real_map.insert("children", 3);
    real_map.insert("cats", 7);
    real_map.insert("samoyeds", 2);
    real_map.insert("pomeranians", 3);
    real_map.insert("akitas", 0);
    real_map.insert("vizslas", 0);
    real_map.insert("goldfish", 5);
    real_map.insert("trees", 3);
    real_map.insert("cars", 2);
    real_map.insert("perfumes", 1);
    puzzle_1(&mut maps, &mut real_map);
    puzzle_2(&mut maps, &mut real_map);
}

fn puzzle_1(maps: &mut Vec<HashMap<&str, i32>>, real_map: &mut HashMap<&str, i32>) {
    'outer: for i in 0..maps.len() {
        let map = maps.get(i).unwrap();
        for &key in map.keys() {
            if real_map.get(key) != map.get(key) {
                continue 'outer;
            }
        }
        println!("Puzzle 1 found Sue {}: {:?}", i + 1, map);
    }
}

fn puzzle_2(maps: &mut Vec<HashMap<&str, i32>>, real_map: &mut HashMap<&str, i32>) {
    'outer: for i in 0..maps.len() {
        let map = maps.get(i).unwrap();
        for &key in map.keys() {
            if key == "cats" || key == "trees" {
                if real_map.get(key) >= map.get(key) {
                    continue 'outer;
                }
            } else if key == "pomeranians" || key == "goldfish" {
                if real_map.get(key) <= map.get(key) {
                    continue 'outer;
                }
            } else if real_map.get(key) != map.get(key) {
                continue 'outer;
            }
        }
        println!("Puzzle 2 found Sue {}: {:?}", i + 1, map);
    }
}
