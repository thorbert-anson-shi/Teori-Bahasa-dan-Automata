use itertools::Itertools;
use lexical_sort::StringSort;
use utils::{check_rec, Enumerable};
use utils::{compare_proper, expon_lang, expon_lang_func};

fn main() {
    l21();
    println!();
    l25();
    println!();
    l36();
    println!();
    l48();
    println!();
    l59();
}

impl Enumerable for String {
    fn count_char(&self, c: &char) -> usize {
        self.chars().filter(|char| char == c).count()
    }
}

fn l21() {
    let a: Vec<&str> = vec!["ac", "c", "bab", "bcaa"];
    let b: Vec<&str> = vec!["ab", "aca", "b", "ccab"];
    let c: Vec<&str> = vec!["ab", "ca"];

    let a_cb = expon_lang(&a, 3);
    let b_sq = expon_lang(&b, 2);
    let c_star: Vec<String> = (0..6).map(|rep| expon_lang(&c, rep)).concat();

    let mut concatenated_lang: Vec<String> = a_cb
        .iter()
        .cartesian_product(b_sq)
        .map(|pair| format!("{}{}", pair.0, pair.1))
        .collect();

    concatenated_lang.retain(|word| !c_star.contains(word));

    concatenated_lang.string_sort(compare_proper);

    concatenated_lang[0..10]
        .iter()
        .for_each(|word| println!("{}", word));
}

fn l25() {
    let l9 = vec!["ab", "ca"];
    let l10 = vec!["ac", "cb"];
    let l3 = vec!["bb", "bac", "a", "cbab"];
    let l4 = vec!["ca", "aab", "b", "caac"];

    let mut l9_star: Vec<String> = (0..5).map(|rep| expon_lang(&l9, rep)).concat();

    let mut l3_u_l4 = l3.clone();
    l3_u_l4.extend(l4.iter().cloned());
    let l3_u_l4_star: Vec<String> = (0..3).map(|rep| expon_lang(&l3_u_l4, rep)).concat();

    let l10_concat_union: Vec<String> = l10
        .iter()
        .cartesian_product(l3_u_l4_star.iter())
        .map(|pair| format!("{}{}", *pair.0, pair.1))
        .collect();

    l9_star.retain(|word| !l10_concat_union.contains(word));

    let mut iter = l9_star.into_iter().filter(|word| word.len() >= 5);

    for _ in 0..10 {
        println!("{}", iter.next().expect("Shit"));
    }
}

fn l36() {
    let l5 = vec!["bc", "c", "bca", "cbac"];
    let l2 = vec!["ac", "c", "bab", "bcaa"];

    let l5_star = expon_lang_func(&l5, 4);

    let l5_star_reversed: Vec<String> = l5_star
        .iter()
        .map(|word| word.chars().rev().collect::<String>())
        .collect();

    let l2_star = expon_lang_func(&l2, 4);

    let mut merged: Vec<String> = l5_star_reversed
        .iter()
        .cartesian_product(l2_star)
        .map(|pair| format!("{}{}", pair.0, pair.1))
        .collect();

    merged.string_sort(compare_proper);

    let mut results: Vec<String> = Vec::new();

    let mut iter = merged.into_iter().filter(|word| word.len() >= 5);

    while results.len() < 10 {
        let curr = iter.next().expect("Shit");

        if !results.contains(&curr) {
            results.push(curr);
        }
    }

    results.iter().for_each(|word| println!("{}", word));
}

fn l48() {
    let lang = vec!["a", "babb"];

    let lang_star = expon_lang_func(&lang, 10);

    let mut filtered: Vec<String> = lang_star
        .into_iter()
        .filter(|word| 3 * word.count_char(&'a') >= 5 * word.count_char(&'b') + 3)
        .filter(|word| word.len() >= 5)
        .collect();

    filtered.string_sort(compare_proper);

    filtered[0..10].iter().for_each(|word| println!("{}", word));
}

// Start from the whole word, and check if the letter count fits within constraints
// If not, skip
// If yes, depending on whether it's a prefix or suffix, trim the end of the word
// Recurse
// If it's yes all the way down, then accept string
fn l59() {
    let lang = vec!["a", "b"];

    let lang_star = expon_lang_func(&lang, 10);

    let filtered = lang_star
        .into_iter()
        .filter(|word| word.len() >= 5)
        .filter(|word| check_rec(word))
        .collect::<Vec<String>>();

    filtered[0..10].iter().for_each(|word| println!("{}", word));
}

mod utils {
    use std::cmp::Ordering;

    pub trait Enumerable {
        fn count_char(&self, c: &char) -> usize;
    }

    pub fn check_rec(s: &str) -> bool {
        if s.len() == 0 {
            return true;
        }

        let ref_str = s.to_string();

        if 3 * ref_str.count_char(&'a') >= 6 * ref_str.count_char(&'b') {
            return check_rec(&s[1..]);
        } else {
            return false;
        }
    }

    pub fn expon_lang_func(lang: &Vec<&str>, repetitions: i8) -> Vec<String> {
        std::iter::once(vec!["".to_string()])
            .chain((0..repetitions).scan(vec!["".to_string()], |acc, _| {
                let next = acc
                    .iter()
                    .flat_map(|prefix| {
                        lang.iter()
                            .map(move |&suffix| format!("{}{}", prefix, suffix))
                    })
                    .collect::<Vec<_>>();
                *acc = next.clone();
                Some(next)
            }))
            .flatten()
            .collect()
    }

    pub fn expon_lang(lang: &Vec<&str>, repetitions: i8) -> Vec<String> {
        if repetitions == 0 {
            return vec!["".to_string()];
        };

        let mut res: Vec<String> = lang.iter().map(|&s| s.to_string()).collect();

        for _i in 1..repetitions {
            let mut temp: Vec<String> = Vec::new();
            for t in &res {
                for word in lang {
                    temp.push(format!("{}{}", t, word));
                }
            }
            res = temp;
        }

        res
    }

    pub fn compare_proper(a: &str, b: &str) -> Ordering {
        // If a is first in order, return true
        if a.len() == b.len() {
            return match compare_proper_rec(a, b) {
                true => Ordering::Less,
                false => Ordering::Greater,
            };
        }

        match a.len() > b.len() {
            true => Ordering::Greater,
            false => Ordering::Less,
        }
    }

    fn compare_proper_rec(a: &str, b: &str) -> bool {
        if a.len() == 0 {
            return false;
        }

        if a.chars().nth(0) > b.chars().nth(0) {
            false
        } else if a.chars().nth(0) < b.chars().nth(0) {
            true
        } else {
            compare_proper_rec(a[1..].as_ref(), b[1..].as_ref())
        }
    }
}
