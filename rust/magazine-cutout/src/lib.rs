// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut hashmap = HashMap::new();
    for iter in magazine {
        let p = hashmap.entry(*iter).or_insert(0);
        *p += 1;
    }
    for iter in note {
        let p = hashmap.entry(*iter).or_insert(0);
        *p -= 1;
        if *p < 0 {
            return false;
        }
    }
    return true;
}
