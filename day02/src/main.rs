use std::fs;

fn main() {
    let input = fs::read_to_string("day02/src/input.txt").unwrap();
    let mut dimensions = vec![];
    for s in input.split('\n') {
        let dimension: Vec<usize> = s
            .split('x')
            .map(|x| x.parse::<usize>().expect("input.txt incorrectly formatted"))
            .collect();
        dimensions.push(dimension);
    }
    let mut area_wrapping_paper = 0;
    for dimension in &dimensions {
        let l = dimension[0];
        let w = dimension[1];
        let h = dimension[2];
        let areas = vec![l * w, w * h, h * l];
        let slack = areas.iter().min().unwrap();
        area_wrapping_paper += 2 * (l * w + w * h + h * l) + *slack;
    }
    println!(
        "Total area of wrapping paper needed: {} square meters",
        area_wrapping_paper
    );

    let mut ribbon_length = 0;
    for dimension in &dimensions {
        let mut vec = vec![dimension[0], dimension[1], dimension[2]];
        vec.sort();
        let a = vec[0];
        let b = vec[1];
        ribbon_length += 2 * a + 2 * b + dimension[0] * dimension[1] * dimension[2];
    }
    println!("Total ribbon length required: {} meters", ribbon_length);
}
