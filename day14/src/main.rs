use std::cmp::max;
use std::fs;

fn main() {
    let input = fs::read_to_string("day14/src/input.txt").unwrap();
    let mut reindeers = vec![];
    for line in input.lines() {
        let words: Vec<&str> = line.split(' ').collect();
        reindeers.push(Reindeer::new(
            words[0],
            words[3].parse::<u32>().unwrap(),
            words[6].parse::<u32>().unwrap(),
            words[13].parse::<u32>().unwrap(),
        ));
    }
    solve_puzzle_1(&reindeers);
    solve_puzzle_2(&reindeers);
}

fn solve_puzzle_1(reindeers_input: &Vec<Reindeer>) {
    let mut reindeers = reindeers_input.clone();
    for _ in 0..2504 {
        for reindeer in &mut reindeers {
            reindeer.step();
        }
    }
    let best_reindeer = reindeers
        .iter()
        .max_by_key(|r| r.distance)
        .expect("no best reindeer found");
    println!(
        "Best: {}, distance: {}",
        best_reindeer.name, best_reindeer.distance
    );
}

fn solve_puzzle_2(input: &Vec<Reindeer>) {
    let mut reindeers = input.clone();
    for _ in 0..2504 {
        for reindeer in &mut reindeers {
            reindeer.step();
        }
        let best_distance = reindeers
            .iter()
            .map(|r| r.distance)
            .fold(0, |a, b| max(a, b));
        let winning_names: Vec<&str> = reindeers
            .iter()
            .filter(|r| r.distance == best_distance)
            .map(|r| r.name)
            .collect();

        for reindeer in &mut reindeers {
            if winning_names.contains(&reindeer.name) {
                reindeer.score += 1;
            }
        }
    }

    println!(
        "Best: {:?}",
        reindeers.into_iter().max_by_key(|r| r.score).unwrap()
    );
}

#[derive(Clone, Debug)]
enum State {
    Flying,
    Resting,
}

#[derive(Clone, Debug)]
struct Reindeer<'a> {
    name: &'a str,
    speed: u32,
    flying_duration: u32,
    resting_duration: u32,
    state: State,
    countdown: u32,
    distance: u32,
    score: u32,
}

impl<'a> Reindeer<'a> {
    fn new(name: &'a str, speed: u32, flying_duration: u32, resting_duration: u32) -> Self {
        Reindeer::<'a> {
            name,
            speed,
            flying_duration,
            resting_duration,
            state: State::Flying,
            countdown: flying_duration,
            distance: 0,
            score: 0,
        }
    }

    fn step(&mut self) {
        if self.countdown == 0 {
            match self.state {
                State::Flying => {
                    self.state = State::Resting;
                    self.countdown = self.resting_duration;
                }
                State::Resting => {
                    self.state = State::Flying;
                    self.countdown = self.flying_duration;
                }
            }
        }
        match self.state {
            State::Flying => {
                self.distance += self.speed;
            }
            State::Resting => {}
        }
        self.countdown -= 1;
    }
}
