use rand::{self, Rng};
use std::ops::Range;

pub fn roll(number_of_dice: usize, max: usize, min: usize) -> usize {
    let mut vec: Vec<usize> = Vec::with_capacity(number_of_dice);
    vec.resize(number_of_dice, 0);
    return self::sum(vec, min, max);
}

// FIXME: 責務過多。乱数生成を切り出したい
fn sum(v: Vec<usize>, min: usize, max: usize) -> usize {
    let mut sum = 0;
    let mut rng = rand::thread_rng();

    for _ in &v {
        let range = Range {
            start: min,
            end: max,
        };
        let random = rng.gen_range(range);
        println!("dice: {:?}", random);
        sum += random;
        println!("sum: {:?}", sum);
    }
    sum
}
