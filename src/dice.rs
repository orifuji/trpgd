use rand::{self, Rng};
use std::ops::Range;

pub fn roll(number_of_dice: usize, max: usize, min: usize) -> usize {
    let vec: Vec<u64> = Vec::with_capacity(number_of_dice);
    let random_values = vec.iter().map(|_| self::random(min, max)).collect();
    return self::sum(random_values);
}

fn random(min: usize, max: usize) -> usize {
    let mut rng = rand::thread_rng();
    let range = Range {
        start: min,
        end: max,
    };
    return rng.gen_range(range);
}

fn sum(v: Vec<usize>) -> usize {
    let mut sum = 0;
    for &i in &v {
        println!("{}", sum.to_string());
        sum += i;
    }
    sum
}
