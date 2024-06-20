use std::collections::HashMap;

use rayon::prelude::*;

use crate::entropy::{
    brain::Word,
    feedback::{Feedback, FB},
};

pub fn suggest_word(options: &Vec<Word>, valid: &Vec<Word>) -> Word {
    let results = find_entropies(options, valid);
    results.last().expect("results should not be empty").0
}

pub fn find_entropies(options: &Vec<Word>, valid: &Vec<Word>) -> Vec<(Word, f64)> {
    //Although we consider putting in words we are know a  wrong.
    //If they have a higher entropy that are worth considering
    let mut results: Vec<(Word, f64)> = valid
        .par_iter()
        .map(|x| (*x, find_entropy(*x, options)))
        .collect();
    results.sort_by(|a, b| a.1.partial_cmp(&b.1).expect("Dont worry about Nan"));
    results
}

pub fn find_entropy(word: Word, options: &Vec<Word>) -> f64 {
    let mut feedback_count: HashMap<[FB; 5], usize> = HashMap::new();
    let size: f64 = options.len() as f64;
    for o in options {
        let fb = Feedback::from_guess(&word, o).mask();
        let count = feedback_count.get(&fb).unwrap_or(&0);
        feedback_count.insert(fb, count + 1);
    }
    feedback_count
        .values()
        .map(|x| {
            let p: f64 = *x as f64 / size;
            p * (1. / p).log2()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanity_check_1() {
        let result = find_entropies(
            &vec![['a', 'a', 'a', 'a', 'a'], ['a', 'a', 'b', 'a', 'a']],
            &vec![['a', 'a', 'a', 'a', 'a'], ['a', 'a', 'b', 'a', 'a']],
        );
        assert_eq!(result[0].1, 1.0);
        assert_eq!(result[1].1, 1.0);
    }

    #[test]
    fn test_sanity_check_2() {
        let result = find_entropies(
            &vec![
                ['a', 'a', 'a', 'a', 'a'],
                ['a', 'a', 'b', 'a', 'a'],
                ['z', 'z', 'z', 'z', 'z'],
            ],
            &vec![
                ['a', 'a', 'a', 'a', 'a'],
                ['a', 'a', 'b', 'a', 'a'],
                ['z', 'z', 'z', 'z', 'z'],
            ],
        );
        dbg![&result];
        assert!(result[0].1 < result[1].1);
        assert!(result[0].1 < result[2].1);
        assert_eq!(result[1].1, result[2].1);
    }
}
