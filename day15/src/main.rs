fn main() {
    let n = 100;
    let frosting = [4, -2, 0, 0, 5];
    let candy = [0, 5, -1, 0, 8];
    let butterscotch = [-1, 0, 5, 0, 6];
    let sugar = [0, 0, -2, 2, 1];
    puzzle_1(n, frosting, candy, butterscotch, sugar);
    puzzle_2(n, frosting, candy, butterscotch, sugar);
}

fn puzzle_1(n: i32, frosting: [i32; 5], candy: [i32; 5], butterscotch: [i32; 5], sugar: [i32; 5]) {
    let mut max_combo = [0; 4];
    let mut max_score = 0;
    for i in 0..(n + 1) {
        for j in 0..(n + 1 - i) {
            for k in 0..(n + 1 - i - j) {
                let l = n - i - j - k;
                let mut properties = [0; 4];
                for q in 0..4 {
                    properties[q] =
                        i * frosting[q] + j * candy[q] + k * butterscotch[q] + l * sugar[q];
                    if properties[q] < 0 {
                        properties[q] = 0;
                    }
                }
                let score = properties.iter().fold(1, |a, &b| a * b);
                if score > max_score {
                    max_score = score;
                    max_combo = [i, j, k, l];
                }
            }
        }
    }
    println!(
        "Puzzle 1: max combo {:?}, max score {}",
        max_combo, max_score
    );
}

fn puzzle_2(n: i32, frosting: [i32; 5], candy: [i32; 5], butterscotch: [i32; 5], sugar: [i32; 5]) {
    let mut max_combo = [0; 4];
    let mut max_score = 0;
    for i in 0..(n + 1) {
        for j in 0..(n + 1 - i) {
            for k in 0..(n + 1 - i - j) {
                let l = n - i - j - k;
                let calories = i * frosting[4] + j * candy[4] + k * butterscotch[4] + l * sugar[4];
                if calories != 500 {
                    continue;
                }
                let mut properties = [0; 4];
                for q in 0..4 {
                    properties[q] =
                        i * frosting[q] + j * candy[q] + k * butterscotch[q] + l * sugar[q];
                    if properties[q] < 0 {
                        properties[q] = 0;
                    }
                }
                // println!("Properties: {:?}", properties);
                let score = properties.iter().fold(1, |a, &b| a * b);
                if score > max_score {
                    max_score = score;
                    max_combo = [i, j, k, l];
                }
            }
        }
    }
    println!(
        "Puzzle 2: max combo {:?}, max score {}",
        max_combo, max_score
    );
}
