use std::fs;

#[derive(Debug)]
enum Action {
    TurnOff((usize, usize), (usize, usize)),
    TurnOn((usize, usize), (usize, usize)),
    Toggle((usize, usize), (usize, usize)),
}

fn main() {
    let input = fs::read_to_string("day06/src/input.txt").unwrap();
    let instructions = input.split('\n');
    let mut actions = vec![];
    for instruction in instructions {
        let words: Vec<&str> = instruction.split(' ').collect();
        if (words[0], words[1]) == ("turn", "off") {
            let a: Vec<usize> = words[2].split(',').map(|x| x.parse().unwrap()).collect();
            let b: Vec<usize> = words[4].split(',').map(|x| x.parse().unwrap()).collect();
            actions.push(Action::TurnOff((a[0], a[1]), (b[0], b[1])));
        } else if (words[0], words[1]) == ("turn", "on") {
            let a: Vec<usize> = words[2].split(',').map(|x| x.parse().unwrap()).collect();
            let b: Vec<usize> = words[4].split(',').map(|x| x.parse().unwrap()).collect();
            actions.push(Action::TurnOn((a[0], a[1]), (b[0], b[1])));
        } else if words[0] == "toggle" {
            let a: Vec<usize> = words[1].split(',').map(|x| x.parse().unwrap()).collect();
            let b: Vec<usize> = words[3].split(',').map(|x| x.parse().unwrap()).collect();
            actions.push(Action::Toggle((a[0], a[1]), (b[0], b[1])));
        }
    }
    solve_puzzles(actions);
}

fn solve_puzzles(actions: Vec<Action>) {
    let mut lights_1 = [[false; 1000]; 1000];
    let mut lights_2 = [[0; 1000]; 1000];
    for action in actions {
        match action {
            Action::TurnOn((a, b), (c, d)) => {
                for x in a..(c + 1) {
                    for y in b..(d + 1) {
                        lights_1[x][y] = true;
                        lights_2[x][y] += 1;
                    }
                }
            }
            Action::TurnOff((a, b), (c, d)) => {
                for x in a..(c + 1) {
                    for y in b..(d + 1) {
                        lights_1[x][y] = false;
                        lights_2[x][y] = std::cmp::max(lights_2[x][y] - 1, 0);
                    }
                }
            }
            Action::Toggle((a, b), (c, d)) => {
                for x in a..(c + 1) {
                    for y in b..(d + 1) {
                        lights_1[x][y] = !lights_1[x][y];
                        lights_2[x][y] += 2;
                    }
                }
            }
        }
    }
    let mut on_lights = 0;
    let mut total_brightness = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if lights_1[x][y] {
                on_lights += 1;
            }
            total_brightness += lights_2[x][y];
        }
    }
    println!("Puzzle 1: there are {} lights turned on.", on_lights);
    println!("Puzzle 2: the total brightness is {}.", total_brightness);
}
