use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    let lowerword = word.to_lowercase();
    for iter in possible_anagrams.into_iter() {
        if word.len() != iter.len() {
            continue;
        }
        if word.to_string() == iter.to_string() {
            continue;
        }
        let lowerstr = iter.to_lowercase();
        if lowerstr == lowerword {
            continue;
        }
        let mut hashword = HashMap::new();
        for ch in lowerstr.chars() {
            let entry = hashword.entry(ch).or_insert(0);
            *entry += 1;
        }
        let mut flag = false;
        for ch in lowerword.chars() {
            let entry = hashword.entry(ch).or_insert(0);
            *entry -= 1;
            if *entry < 0 {
                flag = true;
                break;
            }
        }
        if flag {
            continue;
        } else {
            result.insert(*iter);
        }
    }
    return result;
}
