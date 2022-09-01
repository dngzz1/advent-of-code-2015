use std::collections::{HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("day19/src/input.txt").unwrap();
    let mut replacements = HashSet::new();
    for line in input.lines() {
        let words = line.split(" => ").collect::<Vec<&str>>();
        replacements.insert((words[0], words[1]));
    }
    let initial_molecule = "ORnPBPMgArCaCaCaSiThCaCaSiThCaCaPBSiRnFArRnFArCaCaSiThCaCaSiThCaCaCaCaCaCaSiRnFYFArSiRnMgArCaSiRnPTiTiBFYPBFArSiRnCaSiRnTiRnFArSiAlArPTiBPTiRnCaSiAlArCaPTiTiBPMgYFArPTiRnFArSiRnCaCaFArRnCaFArCaSiRnSiRnMgArFYCaSiRnMgArCaCaSiThPRnFArPBCaSiRnMgArCaCaSiThCaSiRnTiMgArFArSiThSiThCaCaSiRnMgArCaCaSiRnFArTiBPTiRnCaSiAlArCaPTiRnFArPBPBCaCaSiThCaPBSiThPRnFArSiThCaSiThCaSiThCaPTiBSiRnFYFArCaCaPRnFArPBCaCaPBSiRnTiRnFArCaPRnFArSiRnCaCaCaSiThCaRnCaFArYCaSiRnFArBCaCaCaSiThFArPBFArCaSiRnFArRnCaCaCaFArSiRnFArTiRnPMgArF";
    let result = transform_one_many(initial_molecule, &replacements);
    println!("Puzzle 1: number of different molecules = {}", result.len());

}}

fn transform_one_one(molecule:  &str, replacement: (&str, &str)) -> HashSet<String> {
    let mut result = HashSet::new();
    let split_molecule = molecule.split(replacement.0).collect::<Vec<&str>>();
    let n = split_molecule.len();
    for i in 1..n {
        let duet = vec![split_molecule[0..i].join(replacement.0), split_molecule[i..n].join(replacement.0)];
        result.insert(duet.join(replacement.1));
    }
    return result;
}

fn transform_one_many(molecule: &str, replacements: &HashSet<(&str,&str)>) -> HashSet<String> {
    let mut result = HashSet::new();
    for &replacement in replacements {
        result.extend(transform_one_one(molecule, replacement));
    }
    return result;
}

fn transform_many_many(molecules: &HashSet<String>, replacements: &HashSet<(&str,&str)>) -> HashSet<String> {
    let mut result = HashSet::new();
    for molecule in molecules {
        result.extend(transform_one_many(molecule, replacements));
    }
    return result;
}